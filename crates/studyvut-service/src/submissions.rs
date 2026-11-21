//! The user's submitted work as the window gets it, and its files on the
//! disk: the app embeds them, so a file to open is first copied into the
//! cache folder (`<data>/cache/submissions/…`) and opened from there.

use std::path::{Path, PathBuf};

use serde::Serialize;
use studyvut_core::content::{Files, Subject};
use studyvut_core::submissions::{self, Kind, Role, Submission, SubmissionFile};

/// A submission in the list of a subject's "Submitted work".
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmissionRow {
    pub subject: String,
    pub id: String,
    pub title: String,
    pub kind: Kind,
    pub task: Option<String>,
    /// `YYYY-MM-DD`.
    pub submitted: Option<String>,
    pub files: usize,
    /// Bytes of all its files.
    pub size: u64,
}

/// One submission with its files and its links into the subject.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmissionDetail {
    pub subject: String,
    pub id: String,
    pub title: String,
    pub kind: Kind,
    pub task: Option<String>,
    pub measured: Option<String>,
    pub submitted: Option<String>,
    /// Markdown.
    pub summary: Option<String>,
    pub files: Vec<FileView>,
    pub chapters: Vec<ChapterLink>,
    pub topics: Vec<TopicLink>,
    /// The questions of `topics`, for "practise the topics of this work"
    /// (`SetSpec::Questions`).
    pub practice: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct FileView {
    /// What `open_submission_file` and the other file commands take.
    pub index: usize,
    pub path: String,
    pub role: Role,
    pub label: Option<String>,
    pub size: u64,
    /// The system app may open it (a document); otherwise only "show in
    /// folder" and "save a copy".
    pub openable: bool,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChapterLink {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct TopicLink {
    pub topic: String,
    pub questions: usize,
}

pub(crate) fn row(s: &Submission) -> SubmissionRow {
    SubmissionRow {
        subject: s.subject.clone(),
        id: s.id.clone(),
        title: s.title.clone(),
        kind: s.kind,
        task: s.task.clone(),
        submitted: s.submitted.clone(),
        files: s.files.len(),
        size: s.size(),
    }
}

/// `subject` is the submission's subject (the loader only keeps
/// submissions whose subject exists).
pub(crate) fn detail(s: &Submission, subject: Option<&Subject>) -> SubmissionDetail {
    let chapters = subject.map_or_else(Vec::new, |subj| {
        s.chapters
            .iter()
            .filter_map(|c| subj.chapter(c))
            .map(|c| ChapterLink {
                id: c.id.clone(),
                title: c.title.clone(),
            })
            .collect()
    });
    let topics = s
        .topics
        .iter()
        .map(|t| TopicLink {
            topic: t.clone(),
            questions: subject.map_or(0, |subj| subj.questions().filter(|q| &q.topic == t).count()),
        })
        .collect();
    let practice = subject.map_or_else(Vec::new, |subj| {
        subj.questions()
            .filter(|q| s.topics.contains(&q.topic))
            .map(|q| q.id.clone())
            .collect()
    });
    SubmissionDetail {
        subject: s.subject.clone(),
        id: s.id.clone(),
        title: s.title.clone(),
        kind: s.kind,
        task: s.task.clone(),
        measured: s.measured.clone(),
        submitted: s.submitted.clone(),
        summary: s.summary.clone(),
        files: s
            .files
            .iter()
            .enumerate()
            .map(|(index, f)| FileView {
                index,
                path: f.path.clone(),
                role: f.role,
                label: f.label.clone(),
                size: f.size,
                openable: submissions::openable(&f.path),
            })
            .collect(),
        chapters,
        topics,
        practice,
    }
}

/// The file as a real file on the disk: its copy in `cache`, written when
/// missing or different (a newer build may carry a corrected file).
pub(crate) fn materialize(
    files: &dyn Files,
    cache: &Path,
    s: &Submission,
    f: &SubmissionFile,
) -> std::io::Result<PathBuf> {
    let bytes = files
        .read(&s.file_path(f))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, s.file_path(f)))?;
    let mut path = cache.join(&s.subject).join(&s.id);
    path.extend(f.path.split('/'));
    if std::fs::read(&path).ok().as_deref() == Some(bytes.as_slice()) {
        return Ok(path);
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!("part-{}", std::process::id()));
    std::fs::write(&tmp, &bytes)?;
    std::fs::rename(&tmp, &path)?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use studyvut_core::content::MemFiles;

    use crate::paths::Paths;
    use crate::service::{ContentSource, Service, ServiceConfig, ServiceError};

    fn content() -> MemFiles {
        let quiz = "name = \"M\"\n\
            [[question]]\nid = \"T1\"\ntopic = \"Tření\"\ntext = \"x\"\noptions = [\"a\", \"b\"]\nanswer = 0\n\
            [[question]]\nid = \"T2\"\ntopic = \"Tření\"\ntext = \"y\"\noptions = [\"a\", \"b\"]\nanswer = 1\n";
        MemFiles::new()
            .with(
                "bpc-fy1/subject.toml",
                "code = \"BPC-FY1\"\nname = \"Fyzika 1\"\nsemester = \"ZS 2026/2027\"",
            )
            .with("bpc-fy1/quizzes/mech.toml", quiz)
            .with(
                "bpc-fy1/chapters/04-treni.md",
                "+++\ntopics = [\"Tření\"]\n+++\n# Tření\n",
            )
    }

    const LAB: &str = r#"
title = "Nakloněná rovina"
kind = "protocol"
task = 5
submitted = 2026-10-21
chapters = ["04-treni"]
topics = ["Tření"]

[[file]]
path = "protokol.pdf"
role = "final"

[[file]]
path = "zdroj/vypocet.py"
role = "source"
"#;

    fn submissions() -> MemFiles {
        MemFiles::new()
            .with("bpc-fy1/lab-05/submission.toml", LAB)
            .with("bpc-fy1/lab-05/protokol.pdf", "%PDF-1.7 v1")
            .with("bpc-fy1/lab-05/zdroj/vypocet.py", "print(1)")
    }

    fn service(dir: &std::path::Path, subs: MemFiles) -> Service {
        Service::start(ServiceConfig {
            paths: Paths {
                data_dir: dir.to_owned(),
                custom: true,
            },
            files: Arc::new(content()),
            submissions: Arc::new(subs),
            source: ContentSource::Embedded,
        })
        .expect("service")
    }

    #[test]
    fn submissions_come_with_their_links() -> crate::service::Result<()> {
        let dir = tempfile::tempdir()?;
        let svc = service(dir.path(), submissions());
        assert_eq!(svc.problem_count().errors, 0, "{:?}", svc.problems());
        let rows = svc.submissions(Some("bpc-fy1"));
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].files, rows[0].size), (2, 19));
        assert!(svc.submissions(Some("bpc-ma1")).is_empty());
        assert_eq!(svc.overview(0)?.subjects[0].submissions, 1);

        let d = svc.submission("bpc-fy1", "lab-05")?;
        assert_eq!(d.chapters[0].title, "Tření");
        assert_eq!(d.topics[0].questions, 2);
        assert_eq!(d.practice, vec!["T1", "T2"]);
        let open: Vec<bool> = d.files.iter().map(|f| f.openable).collect();
        assert_eq!(open, vec![true, false], "a script is never opened");
        let e = svc.submission("bpc-fy1", "nic");
        assert_eq!(e.err().map(|e| e.code()), Some("unknown_submission"));
        Ok(())
    }

    #[test]
    fn files_are_copied_to_the_cache_and_kept_current() -> crate::service::Result<()> {
        let dir = tempfile::tempdir()?;
        let svc = service(dir.path(), submissions());
        let pdf = svc.submission_file("bpc-fy1", "lab-05", 0, true)?;
        assert!(pdf.starts_with(dir.path().join("cache/submissions")));
        assert_eq!(std::fs::read_to_string(&pdf)?, "%PDF-1.7 v1");
        assert!(matches!(
            svc.submission_file("bpc-fy1", "lab-05", 1, true),
            Err(ServiceError::NotOpenable(_))
        ));
        assert!(svc.submission_file("bpc-fy1", "lab-05", 1, false).is_ok());
        assert!(matches!(
            svc.submission_file("bpc-fy1", "lab-05", 7, false),
            Err(ServiceError::UnknownFile(_))
        ));

        // A newer build with a corrected file replaces the cached copy.
        let newer = submissions().with("bpc-fy1/lab-05/protokol.pdf", "%PDF-1.7 v2");
        let svc = service(dir.path(), newer);
        let pdf = svc.submission_file("bpc-fy1", "lab-05", 0, true)?;
        assert_eq!(std::fs::read_to_string(&pdf)?, "%PDF-1.7 v2");

        let (name, bytes) = svc.submission_bytes("bpc-fy1", "lab-05", 1)?;
        assert_eq!(
            (name.as_str(), bytes.as_slice()),
            ("vypocet.py", &b"print(1)"[..])
        );
        Ok(())
    }

    #[test]
    fn problems_of_submissions_are_content_problems() -> crate::service::Result<()> {
        let dir = tempfile::tempdir()?;
        let svc = service(
            dir.path(),
            submissions().with("bpc-fy1/lab-05/navic.pdf", "%PDF"),
        );
        let p = svc.problems();
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].path, "submissions/bpc-fy1/lab-05/navic.pdf");
        Ok(())
    }
}
