//! The service: content, settings and progress behind one handle, and the
//! actions every front end (the app's commands, the CLI) goes through.
//!
//! Progress is kept for what helps learning (the latest answer per question
//! and its review schedule), not for statistics: the user chose an app
//! without success statistics (2026-11-21).

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use serde::Serialize;
use studyvut_core::content::{
    self, Catalog, Files, Problem, QuestionKind, SetRule, Severity, Subject, markdown::image_refs,
};
use studyvut_core::quiz::{self, Personal, Response, SetSpec, Verdict};
use studyvut_core::store::{Answer, QuestionState, SessionRow, Store, StoreError};
use studyvut_core::{local_day, now_ms};
use tracing::{info, warn};

use crate::paths::Paths;
use crate::settings::Settings;
use crate::views::{
    ChapterPayload, ChapterView, Overview, ProblemCount, SessionPayload, SetView, SubjectView,
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("no subject {0:?}")]
    UnknownSubject(String),
    #[error("no question {0:?}")]
    UnknownQuestion(String),
    #[error("no chapter {0:?}")]
    UnknownChapter(String),
    #[error("the set has no questions")]
    EmptySet,
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl ServiceError {
    /// Machine-readable reason a front end can translate.
    pub fn code(&self) -> &'static str {
        match self {
            ServiceError::UnknownSubject(_) => "unknown_subject",
            ServiceError::UnknownQuestion(_) => "unknown_question",
            ServiceError::UnknownChapter(_) => "unknown_chapter",
            ServiceError::EmptySet => "empty_set",
            ServiceError::Store(StoreError::NoSession(_)) => "no_session",
            ServiceError::Store(_) => "store",
            ServiceError::Io(_) => "io",
        }
    }
}

pub type Result<T> = std::result::Result<T, ServiceError>;

/// Where the content comes from.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContentSource {
    /// The copy built into the app.
    Embedded,
    /// A folder on the disk (debug builds, `STUDYVUT_CONTENT_DIR`, the CLI).
    Folder { path: PathBuf },
}

pub struct ServiceConfig {
    pub paths: Paths,
    pub files: Arc<dyn Files>,
    pub source: ContentSource,
}

struct Loaded {
    catalog: Catalog,
    problems: Vec<Problem>,
}

pub struct Service {
    pub paths: Paths,
    pub source: ContentSource,
    /// No settings file existed.
    pub first_run: bool,
    /// Where a damaged database was moved at the start.
    pub db_replaced: Option<PathBuf>,
    files: Arc<dyn Files>,
    content: RwLock<Arc<Loaded>>,
    settings: RwLock<Settings>,
    store: Mutex<Store>,
    rng: Mutex<fastrand::Rng>,
}

impl Service {
    pub fn start(config: ServiceConfig) -> Result<Self> {
        config.paths.create()?;
        let (settings, first_run) = Settings::load(&config.paths.settings_file());
        let (store, db_replaced) = Store::open_or_replace(&config.paths.db_file())?;
        if let Some(aside) = &db_replaced {
            warn!(
                "the progress database was damaged; moved it to {}",
                aside.display()
            );
        }
        let loaded = load(config.files.as_ref());
        Ok(Self {
            paths: config.paths,
            source: config.source,
            first_run,
            db_replaced,
            files: config.files,
            content: RwLock::new(Arc::new(loaded)),
            settings: RwLock::new(settings),
            store: Mutex::new(store),
            rng: Mutex::new(fastrand::Rng::new()),
        })
    }

    /// A service on in-memory progress, for tests and CLI checks.
    pub fn in_memory(files: Arc<dyn Files>) -> Result<Self> {
        Ok(Self {
            paths: Paths {
                data_dir: PathBuf::new(),
                custom: true,
            },
            source: ContentSource::Embedded,
            first_run: false,
            db_replaced: None,
            content: RwLock::new(Arc::new(load(files.as_ref()))),
            files,
            settings: RwLock::new(Settings::default()),
            store: Mutex::new(Store::open_in_memory()?),
            rng: Mutex::new(fastrand::Rng::with_seed(1)),
        })
    }

    fn loaded(&self) -> Arc<Loaded> {
        self.content.read().clone()
    }

    pub fn catalog(&self) -> Catalog {
        self.loaded().catalog.clone()
    }

    pub fn problems(&self) -> Vec<Problem> {
        self.loaded().problems.clone()
    }

    pub fn problem_count(&self) -> ProblemCount {
        count(&self.loaded().problems)
    }

    /// Read the content again (a folder sees edits; the embedded copy is the same).
    pub fn reload_content(&self) -> ProblemCount {
        let loaded = load(self.files.as_ref());
        let n = count(&loaded.problems);
        *self.content.write() = Arc::new(loaded);
        n
    }

    pub fn settings(&self) -> Settings {
        self.settings.read().clone()
    }

    pub fn save_settings(&self, settings: Settings) -> Result<Settings> {
        let settings = settings.validated();
        settings.save(&self.paths.settings_file())?;
        *self.settings.write() = settings.clone();
        Ok(settings)
    }

    /// The Dashboard: the programmes, and every subject with its sets.
    pub fn overview(&self, offset_min: i32) -> Result<Overview> {
        let loaded = self.loaded();
        let today = local_day(now_ms(), offset_min);
        let states = self.store.lock().question_states(None)?;
        let mut by_subject: HashMap<&str, Vec<&QuestionState>> = HashMap::new();
        for s in &states {
            by_subject.entry(s.subject.as_str()).or_default().push(s);
        }
        let subjects = loaded
            .catalog
            .subjects
            .iter()
            .map(|s| {
                let mine = by_subject.remove(s.id.as_str()).unwrap_or_default();
                subject_view(s, &personal(mine, today))
            })
            .collect();
        Ok(Overview {
            programs: loaded.catalog.programs.clone(),
            subjects,
            problems: count(&loaded.problems),
        })
    }

    pub fn chapter(&self, subject: &str, chapter: &str) -> Result<ChapterPayload> {
        let loaded = self.loaded();
        let s = find(&loaded.catalog, subject)?;
        let c = s
            .chapter(chapter)
            .ok_or_else(|| ServiceError::UnknownChapter(chapter.to_owned()))?;
        Ok(ChapterPayload {
            subject: s.id.clone(),
            id: c.id.clone(),
            title: c.title.clone(),
            topics: c.topics.clone(),
            svgs: self.svgs(&s.id, image_refs(&c.markdown)),
            markdown: c.markdown.clone(),
            questions: s
                .questions()
                .filter(|q| c.topics.contains(&q.topic))
                .count(),
        })
    }

    /// Start a session: choose its questions and remember it.
    pub fn start_session(
        &self,
        subject: &str,
        spec: &SetSpec,
        offset_min: i32,
    ) -> Result<SessionPayload> {
        let loaded = self.loaded();
        let s = find(&loaded.catalog, subject)?;
        let store = self.store.lock();
        let personal = personal(
            &store.question_states(Some(subject))?,
            local_day(now_ms(), offset_min),
        );
        let ids = quiz::select(s, spec, &personal, &mut self.rng.lock());
        if ids.is_empty() {
            return Err(ServiceError::EmptySet);
        }
        let questions: Vec<_> = ids
            .iter()
            .filter_map(|id| s.question(id))
            .cloned()
            .collect();
        let key = spec.key();
        let id = store.start_session(subject, &key, spec.scored(), questions.len(), now_ms())?;
        let mut refs = Vec::new();
        for q in &questions {
            refs.extend(q.figure.iter().cloned());
            refs.extend(image_refs(&q.text));
            refs.extend(image_refs(&q.explanation));
            if let QuestionKind::Choice { options, .. } = &q.kind {
                for o in options {
                    refs.extend(image_refs(o));
                }
            }
        }
        Ok(SessionPayload {
            id,
            subject: s.id.clone(),
            key,
            svgs: self.svgs(&s.id, refs),
            questions,
        })
    }

    /// Evaluate and record an answer.
    pub fn answer(
        &self,
        session: Option<i64>,
        subject: &str,
        question: &str,
        response: &Response,
        offset_min: i32,
    ) -> Result<Verdict> {
        let loaded = self.loaded();
        let s = find(&loaded.catalog, subject)?;
        let q = s
            .question(question)
            .ok_or_else(|| ServiceError::UnknownQuestion(question.to_owned()))?;
        let verdict = quiz::evaluate(q, response);
        let json = serde_json::to_string(response).unwrap_or_default();
        self.store.lock().record(&Answer {
            session,
            subject,
            question,
            correct: verdict.correct,
            response: &json,
            ts: now_ms(),
            offset_min,
        })?;
        Ok(verdict)
    }

    /// End a session; the result of this one quiz (right answers of all).
    pub fn finish_session(&self, session: i64) -> Result<SessionRow> {
        Ok(self.store.lock().finish_session(session, now_ms())?)
    }

    /// Forget the progress of a subject, or of everything.
    pub fn reset_progress(&self, subject: Option<&str>) -> Result<()> {
        self.store.lock().reset(subject)?;
        info!("progress reset: {}", subject.unwrap_or("all subjects"));
        Ok(())
    }

    /// Every answer as pretty JSON, for an export.
    pub fn export_progress(&self) -> Result<String> {
        let answers = self.store.lock().answers()?;
        let doc = serde_json::json!({
            "format": "studyvut-progress",
            "version": 1,
            "exported": now_ms(),
            "answers": answers,
        });
        Ok(serde_json::to_string_pretty(&doc).unwrap_or_default())
    }

    /// An image of the content (`bpc-fy1/images/x.png`) and its media type.
    pub fn asset(&self, path: &str) -> Option<(Vec<u8>, &'static str)> {
        content::is_image(path)
            .then(|| self.files.read(path))
            .flatten()
            .map(|bytes| (bytes, content::mime_type(path)))
    }

    /// The SVG files among `refs`, read, by their path within the subject.
    fn svgs(&self, subject: &str, refs: Vec<String>) -> BTreeMap<String, String> {
        let mut out = BTreeMap::new();
        for r in refs {
            if !r.to_ascii_lowercase().ends_with(".svg") || out.contains_key(&r) {
                continue;
            }
            if let Some(svg) = self.files.read_string(&format!("{subject}/{r}")) {
                out.insert(r, svg);
            }
        }
        out
    }
}

fn load(files: &dyn Files) -> Loaded {
    let (catalog, problems) = content::load(files);
    let n = count(&problems);
    info!(
        "content: {} subjects, {} errors, {} warnings",
        catalog.subjects.len(),
        n.errors,
        n.warnings
    );
    for p in problems.iter().filter(|p| p.severity == Severity::Error) {
        warn!("{p}");
    }
    Loaded { catalog, problems }
}

fn count(problems: &[Problem]) -> ProblemCount {
    ProblemCount {
        errors: problems
            .iter()
            .filter(|p| p.severity == Severity::Error)
            .count(),
        warnings: problems
            .iter()
            .filter(|p| p.severity == Severity::Warning)
            .count(),
    }
}

fn find<'a>(catalog: &'a Catalog, subject: &str) -> Result<&'a Subject> {
    catalog
        .subject(subject)
        .ok_or_else(|| ServiceError::UnknownSubject(subject.to_owned()))
}

/// The personal sets: questions whose latest answer was wrong, and those due.
fn personal<'a>(states: impl IntoIterator<Item = &'a QuestionState>, today: i64) -> Personal {
    let mut p = Personal::default();
    for s in states {
        if !s.last_correct {
            p.mistakes.insert(s.question.clone());
        }
        if s.due_day <= today {
            p.due.insert(s.question.clone());
        }
    }
    p
}

fn subject_view(s: &Subject, personal: &Personal) -> SubjectView {
    // Questions removed from the content no longer count.
    let mistakes = s
        .questions()
        .filter(|q| personal.mistakes.contains(&q.id))
        .count();
    let due = s
        .questions()
        .filter(|q| personal.due.contains(&q.id))
        .count();
    let questions = s.questions().count();

    let mut sets = Vec::new();
    let mut push = |spec: SetSpec,
                    kind: &'static str,
                    name: String,
                    description: Option<String>,
                    count: usize| {
        sets.push(SetView {
            key: spec.key(),
            spec,
            kind,
            name,
            description,
            count,
        });
    };
    for set in &s.sets {
        let count = match &set.rule {
            SetRule::Tags(tags) => s
                .questions()
                .filter(|q| q.tags.iter().any(|t| tags.contains(t)))
                .count(),
            SetRule::Pick(picks) => picks
                .iter()
                .map(|p| {
                    s.quiz(&p.quiz)
                        .map_or(0, |q| q.questions.len().min(p.count))
                })
                .sum(),
        };
        push(
            SetSpec::Custom { id: set.id.clone() },
            "custom",
            set.name.clone(),
            set.description.clone(),
            count,
        );
    }
    for quiz in &s.quizzes {
        push(
            SetSpec::Quiz {
                id: quiz.id.clone(),
            },
            "quiz",
            quiz.name.clone(),
            quiz.description.clone(),
            quiz.questions.len(),
        );
    }
    if questions > 0 {
        push(SetSpec::Mistakes, "mistakes", String::new(), None, mistakes);
        push(SetSpec::Due, "due", String::new(), None, due);
    }

    SubjectView {
        id: s.id.clone(),
        code: s.code.clone(),
        name: s.name.clone(),
        semester: s.semester.clone(),
        year: s.year,
        programs: s.programs.clone(),
        color: s.color,
        description: s.description.clone(),
        chapters: s
            .chapters
            .iter()
            .map(|c| ChapterView {
                id: c.id.clone(),
                title: c.title.clone(),
                topics: c.topics.clone(),
                questions: s
                    .questions()
                    .filter(|q| c.topics.contains(&q.topic))
                    .count(),
            })
            .collect(),
        sets,
        questions,
        due,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use studyvut_core::content::MemFiles;

    fn service() -> Service {
        let quiz = "name = \"Mechanika\"\n\
            [[question]]\nid = \"M1\"\ntopic = \"T\"\ntext = \"a\"\nfigure = \"images/f.svg\"\noptions = [\"x\", \"y\"]\nanswer = 0\n\
            [[question]]\nid = \"M2\"\ntopic = \"T\"\ntext = \"b ![](images/g.png)\"\noptions = [\"x\", \"y\"]\nanswer = 1\n";
        let files = MemFiles::new()
            .with(
                "programs.toml",
                "[[program]]\nid = \"met\"\ncode = \"BPC-MET\"\nname = \"Mikroelektronika a technologie\"\n",
            )
            .with(
                "fy1/subject.toml",
                "code = \"BPC-FY1\"\nname = \"Fyzika 1\"\nsemester = \"ZS 2026/2027\"\nyear = 1\nprograms = [\"met\"]\n",
            )
            .with("fy1/quizzes/mech.toml", quiz)
            .with("fy1/images/f.svg", "<svg class=\"fig\"/>")
            .with("fy1/images/g.png", vec![0x89, b'P', b'N', b'G'])
            .with(
                "fy1/chapters/01-a.md",
                "+++\ntopics = [\"T\"]\n+++\n# A\n![](images/f.svg)\n",
            );
        Service::in_memory(Arc::new(files)).expect("service")
    }

    #[test]
    fn a_session_runs_from_start_to_result() -> Result<()> {
        let svc = service();
        let spec = SetSpec::Quiz { id: "mech".into() };
        let session = svc.start_session("fy1", &spec, 60)?;
        assert_eq!(session.questions.len(), 2);
        assert_eq!(
            session.svgs.keys().collect::<Vec<_>>(),
            vec!["images/f.svg"],
            "only SVGs are inlined"
        );
        for q in &session.questions {
            let right = if q.id == "M1" { 0 } else { 1 };
            let v = svc.answer(
                Some(session.id),
                "fy1",
                &q.id,
                &Response::Choice { index: right },
                60,
            )?;
            assert!(v.correct);
        }
        let result = svc.finish_session(session.id)?;
        assert_eq!((result.correct, result.total), (2, 2));

        let o = svc.overview(60)?;
        assert_eq!(o.programs.len(), 1);
        let fy1 = &o.subjects[0];
        assert_eq!(
            (fy1.year, fy1.programs.clone()),
            (Some(1), vec!["met".to_owned()])
        );
        assert_eq!((fy1.questions, fy1.due), (2, 0));
        assert_eq!(fy1.chapters[0].questions, 2);
        let mistakes = fy1.sets.iter().find(|s| s.kind == "mistakes").expect("set");
        assert_eq!(mistakes.count, 0);
        Ok(())
    }

    #[test]
    fn mistakes_become_a_set() -> Result<()> {
        let svc = service();
        assert!(matches!(
            svc.start_session("fy1", &SetSpec::Mistakes, 0),
            Err(ServiceError::EmptySet)
        ));
        svc.answer(None, "fy1", "M2", &Response::Choice { index: 0 }, 0)?;
        let s = svc.start_session("fy1", &SetSpec::Mistakes, 0)?;
        assert_eq!(
            s.questions
                .iter()
                .map(|q| q.id.as_str())
                .collect::<Vec<_>>(),
            vec!["M2"]
        );
        let o = svc.overview(0)?;
        let mistakes = o.subjects[0]
            .sets
            .iter()
            .find(|s| s.kind == "mistakes")
            .expect("set");
        assert_eq!(mistakes.count, 1);
        Ok(())
    }

    #[test]
    fn unknown_things_are_errors_with_codes() {
        let svc = service();
        let e = svc.answer(None, "nope", "M1", &Response::Choice { index: 0 }, 0);
        assert_eq!(e.err().map(|e| e.code()), Some("unknown_subject"));
        let e = svc.answer(None, "fy1", "nope", &Response::Choice { index: 0 }, 0);
        assert_eq!(e.err().map(|e| e.code()), Some("unknown_question"));
        let e = svc.chapter("fy1", "nope");
        assert_eq!(e.err().map(|e| e.code()), Some("unknown_chapter"));
    }

    #[test]
    fn chapters_carry_their_svgs_and_assets_are_images_only() -> Result<()> {
        let svc = service();
        let c = svc.chapter("fy1", "01-a")?;
        assert!(c.svgs.contains_key("images/f.svg"));
        assert_eq!(
            svc.asset("fy1/images/g.png").map(|a| a.1),
            Some("image/png")
        );
        assert!(svc.asset("fy1/subject.toml").is_none());
        Ok(())
    }
}
