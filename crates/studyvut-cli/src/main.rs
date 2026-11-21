//! `studyvut-cli`: the app's command-line sibling.
//!
//! - `check`: validate `content/` and `submissions/` (CI runs it; exit code 1 on errors);
//! - `list`: what the content and the submitted work hold;
//! - `stats`: the progress the app recorded;
//! - `paths`: where the app keeps its data.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use studyvut_core::content::{self, DirFiles, QuestionKind, Severity};
use studyvut_core::store::Store;
use studyvut_core::submissions;
use studyvut_service::Paths;

#[derive(Parser)]
#[command(name = "studyvut-cli", version, about = "STUDYVUT on the command line")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check the content and the submitted work; exit code 1 when they have errors.
    Check {
        /// The content folder.
        #[arg(long, env = "STUDYVUT_CONTENT_DIR", default_value = "content")]
        content: PathBuf,
        /// The submitted work (docs/submissions.md); a missing folder is empty.
        #[arg(long, env = "STUDYVUT_SUBMISSIONS_DIR", default_value = "submissions")]
        submissions: PathBuf,
        /// Fail on warnings too.
        #[arg(long)]
        strict: bool,
    },
    /// List the subjects, chapters, quizzes, sets and submitted work.
    List {
        #[arg(long, env = "STUDYVUT_CONTENT_DIR", default_value = "content")]
        content: PathBuf,
        #[arg(long, env = "STUDYVUT_SUBMISSIONS_DIR", default_value = "submissions")]
        submissions: PathBuf,
        /// As JSON.
        #[arg(long)]
        json: bool,
    },
    /// The progress the app recorded, per subject.
    Stats {
        /// The app's data folder (default: the platform's).
        #[arg(long, env = "STUDYVUT_DATA_DIR")]
        data_dir: Option<PathBuf>,
    },
    /// Where the app keeps its settings, progress and logs.
    Paths {
        #[arg(long, env = "STUDYVUT_DATA_DIR")]
        data_dir: Option<PathBuf>,
    },
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("studyvut-cli: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode> {
    match cli.command {
        Command::Check {
            content,
            submissions,
            strict,
        } => check(&content, &submissions, strict),
        Command::List {
            content,
            submissions,
            json,
        } => list(&content, &submissions, json),
        Command::Stats { data_dir } => stats(data_dir),
        Command::Paths { data_dir } => {
            let p = Paths::resolve(data_dir).context("no local data folder on this system")?;
            println!("data folder  {}", p.data_dir.display());
            println!("settings     {}", p.settings_file().display());
            println!("progress     {}", p.db_file().display());
            println!("logs         {}", p.logs_dir().display());
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn content_dir(dir: &PathBuf) -> Result<DirFiles> {
    anyhow::ensure!(dir.is_dir(), "{} is not a folder", dir.display());
    Ok(DirFiles::new(dir))
}

/// `submissions/` may be missing (nothing submitted yet); it is then empty.
fn submissions_dir(dir: &PathBuf) -> Result<DirFiles> {
    anyhow::ensure!(
        dir.is_dir() || !dir.exists(),
        "{} is not a folder",
        dir.display()
    );
    Ok(DirFiles::new(dir))
}

fn check(dir: &PathBuf, submissions_at: &PathBuf, strict: bool) -> Result<ExitCode> {
    let files = content_dir(dir)?;
    let (catalog, mut problems) = content::load(&files);
    let (subs, more) = submissions::load(&submissions_dir(submissions_at)?, &catalog);
    problems.extend(more.into_iter().map(|mut p| {
        p.path = format!("submissions/{}", p.path);
        p
    }));
    for p in &problems {
        eprintln!("{p}");
    }
    let errors = problems
        .iter()
        .filter(|p| p.severity == Severity::Error)
        .count();
    let warnings = problems.len() - errors;
    let questions: usize = catalog.subjects.iter().map(|s| s.questions().count()).sum();
    let chapters: usize = catalog.subjects.iter().map(|s| s.chapters.len()).sum();
    let bytes: u64 = subs.iter().map(|s| s.size()).sum();
    println!(
        "{} subjects, {chapters} chapters, {questions} questions, {} submissions ({:.1} MB): \
         {errors} errors, {warnings} warnings",
        catalog.subjects.len(),
        subs.len(),
        bytes as f64 / f64::from(1 << 20)
    );
    Ok(if errors > 0 || (strict && warnings > 0) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

fn list(dir: &PathBuf, submissions_at: &PathBuf, json: bool) -> Result<ExitCode> {
    let files = content_dir(dir)?;
    let (catalog, _) = content::load(&files);
    let sub_files = submissions_dir(submissions_at)?;
    let (subs, _) = submissions::load(&sub_files, &catalog);
    if json {
        let service =
            studyvut_service::Service::in_memory_with(Arc::new(files), Arc::new(sub_files))?;
        println!(
            "{}",
            serde_json::to_string_pretty(&service.overview(0)?.subjects)?
        );
        return Ok(ExitCode::SUCCESS);
    }
    for s in &catalog.subjects {
        let numeric = s
            .questions()
            .filter(|q| matches!(q.kind, QuestionKind::Numeric { .. }))
            .count();
        println!("{} {}  ({}, {})", s.code, s.name, s.semester, s.id);
        for c in &s.chapters {
            println!("  chapter  {}  {}", c.id, c.title);
        }
        for q in &s.quizzes {
            println!(
                "  quiz     {}  {} ({} questions)",
                q.id,
                q.name,
                q.questions.len()
            );
        }
        for w in subs.iter().filter(|w| w.subject == s.id) {
            println!(
                "  work     {}  {} ({} files, {:.1} MB)",
                w.id,
                w.title,
                w.files.len(),
                w.size() as f64 / f64::from(1 << 20)
            );
        }
        for set in &s.sets {
            println!("  set      {}  {}", set.id, set.name);
        }
        let total = s.questions().count();
        if total > 0 {
            println!(
                "  {total} questions, {numeric} numeric, {} topics",
                s.topics().len()
            );
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn stats(data_dir: Option<PathBuf>) -> Result<ExitCode> {
    let paths = Paths::resolve(data_dir).context("no local data folder on this system")?;
    let db = paths.db_file();
    anyhow::ensure!(
        db.exists(),
        "no progress yet ({} does not exist)",
        db.display()
    );
    let store = Store::open(&db)?;
    let states = store.question_states(None)?;
    let mut subjects: Vec<&str> = states.iter().map(|s| s.subject.as_str()).collect();
    subjects.dedup();
    if subjects.is_empty() {
        println!("no answers yet");
    }
    for subject in subjects {
        let mine: Vec<_> = states.iter().filter(|s| s.subject == subject).collect();
        let right = mine.iter().filter(|s| s.last_correct).count();
        let answered: u32 = mine.iter().map(|s| s.answered).sum();
        println!(
            "{subject:<10} {:>4} questions seen, {right:>4} right last time, {answered:>5} answers",
            mine.len()
        );
    }
    Ok(ExitCode::SUCCESS)
}
