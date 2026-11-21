//! The service: content, settings and progress behind one handle, and the
//! actions every front end (the app's commands, the CLI) goes through.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use serde::Serialize;
use studyvut_core::content::{
    self, Catalog, Files, Problem, QuestionKind, SetRule, Severity, Subject, markdown::image_refs,
};
use studyvut_core::quiz::{self, Personal, Response, SetSpec, Verdict};
use studyvut_core::store::{self, Answer, DayActivity, QuestionState, Store, StoreError};
use studyvut_core::{DAY_MS, local_day, now_ms};
use tracing::{info, warn};

use crate::paths::Paths;
use crate::settings::Settings;
use crate::views::{
    Activity, ChapterPayload, ChapterView, Overview, ProblemCount, Progress, QuestionRow,
    SessionPayload, SessionResult, SetView, SubjectView,
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

    /// The Dashboard: every subject with its sets and progress.
    pub fn overview(&self, offset_min: i32) -> Result<Overview> {
        let loaded = self.loaded();
        let now = now_ms();
        let today = local_day(now, offset_min);
        let store = self.store.lock();
        let states = store.question_states(None)?;
        let mut by_subject: HashMap<&str, Vec<&QuestionState>> = HashMap::new();
        for s in &states {
            by_subject.entry(s.subject.as_str()).or_default().push(s);
        }
        let mut subjects = Vec::new();
        for subject in &loaded.catalog.subjects {
            let states = by_subject.remove(subject.id.as_str()).unwrap_or_default();
            let best = store.best_scores(&subject.id)?;
            subjects.push(subject_view(subject, &states, &best, today));
        }
        let activity = store.activity(now - 400 * DAY_MS, offset_min)?;
        let today_row = activity.iter().find(|d| d.day == today);
        Ok(Overview {
            subjects,
            problems: count(&loaded.problems),
            today,
            answered_today: today_row.map_or(0, |d| d.answered),
            correct_today: today_row.map_or(0, |d| d.correct),
            streak: streak(&activity, today),
            recent: store.recent_sessions(8)?,
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
            scored: spec.scored(),
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

    pub fn finish_session(&self, session: i64) -> Result<SessionResult> {
        let store = self.store.lock();
        let before = store
            .session(session)?
            .ok_or(StoreError::NoSession(session))?;
        let previous = store
            .best_scores(&before.subject)?
            .get(&before.set_key)
            .copied();
        let row = store.finish_session(session, now_ms())?;
        let best = store.best_scores(&row.subject)?.get(&row.set_key).copied();
        let scored = before.finished.is_none() && best.is_some() && best != previous;
        Ok(SessionResult {
            session: row,
            best,
            new_best: scored,
        })
    }

    /// Answers per local day for the last `days` days, today included.
    pub fn activity(&self, days: u32, offset_min: i32) -> Result<Activity> {
        let now = now_ms();
        let today = local_day(now, offset_min);
        let first = today - i64::from(days.max(1)) + 1;
        let rows = self
            .store
            .lock()
            .activity(first * DAY_MS - i64::from(offset_min) * 60_000, offset_min)?;
        let by_day: HashMap<i64, &DayActivity> = rows.iter().map(|d| (d.day, d)).collect();
        Ok((first..=today)
            .map(|day| {
                by_day.get(&day).map_or(
                    DayActivity {
                        day,
                        answered: 0,
                        correct: 0,
                    },
                    |d| (*d).clone(),
                )
            })
            .collect())
    }

    /// Every question of the content with the user's history.
    pub fn questions(&self) -> Result<Vec<QuestionRow>> {
        let loaded = self.loaded();
        let store = self.store.lock();
        let totals: HashMap<(String, String), store::QuestionTotals> = store
            .question_totals()?
            .into_iter()
            .map(|t| ((t.subject.clone(), t.question.clone()), t))
            .collect();
        let states: HashMap<(String, String), QuestionState> = store
            .question_states(None)?
            .into_iter()
            .map(|s| ((s.subject.clone(), s.question.clone()), s))
            .collect();
        let mut out = Vec::new();
        for s in &loaded.catalog.subjects {
            for quiz in &s.quizzes {
                for q in &quiz.questions {
                    let key = (s.id.clone(), q.id.clone());
                    let t = totals.get(&key);
                    let st = states.get(&key);
                    out.push(QuestionRow {
                        subject: s.id.clone(),
                        id: q.id.clone(),
                        quiz: quiz.id.clone(),
                        topic: q.topic.clone(),
                        text: q.text.clone(),
                        answered: t.map_or(0, |t| t.answered),
                        correct: t.map_or(0, |t| t.correct),
                        last_correct: st.map(|s| s.last_correct),
                        last_ts: st.map(|s| s.last_ts),
                        r#box: st.map(|s| s.r#box),
                        due_day: st.map(|s| s.due_day),
                    });
                }
            }
        }
        Ok(out)
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

fn subject_view(
    s: &Subject,
    states: &[&QuestionState],
    best: &HashMap<String, u32>,
    today: i64,
) -> SubjectView {
    let ids: HashSet<&str> = s.questions().map(|q| q.id.as_str()).collect();
    let mine: Vec<&QuestionState> = states
        .iter()
        .copied()
        .filter(|st| ids.contains(st.question.as_str()))
        .collect();
    let personal = personal(mine.iter().copied(), today);
    let progress = Progress {
        questions: ids.len(),
        seen: mine.len(),
        correct: mine.iter().filter(|st| st.last_correct).count(),
        wrong: personal.mistakes.len(),
        due: personal.due.len(),
    };

    let mut sets = Vec::new();
    let mut push = |spec: SetSpec,
                    kind: &'static str,
                    name: String,
                    description: Option<String>,
                    count: usize| {
        let key = spec.key();
        sets.push(SetView {
            best: best.get(&key).copied(),
            key,
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
    if progress.questions > 0 {
        push(
            SetSpec::Mistakes,
            "mistakes",
            String::new(),
            None,
            progress.wrong,
        );
        push(SetSpec::Due, "due", String::new(), None, progress.due);
    }

    SubjectView {
        id: s.id.clone(),
        code: s.code.clone(),
        name: s.name.clone(),
        semester: s.semester.clone(),
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
        progress,
    }
}

/// Consecutive days with answers ending today, or yesterday when today has none yet.
fn streak(activity: &[DayActivity], today: i64) -> u32 {
    let days: HashSet<i64> = activity
        .iter()
        .filter(|d| d.answered > 0)
        .map(|d| d.day)
        .collect();
    let mut day = if days.contains(&today) {
        today
    } else {
        today - 1
    };
    let mut n = 0;
    while days.contains(&day) {
        n += 1;
        day -= 1;
    }
    n
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
                "fy1/subject.toml",
                "code = \"BPC-FY1\"\nname = \"Fyzika 1\"\nsemester = \"ZS 2026/2027\"\n",
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
    fn a_session_runs_from_start_to_best_score() -> Result<()> {
        let svc = service();
        let spec = SetSpec::Quiz { id: "mech".into() };
        let session = svc.start_session("fy1", &spec, 60)?;
        assert_eq!(session.questions.len(), 2);
        assert_eq!(
            session.svgs.keys().collect::<Vec<_>>(),
            vec!["images/f.svg"],
            "only SVGs are inlined"
        );

        let right = |id: &str| if id == "M1" { 0 } else { 1 };
        for q in &session.questions {
            let v = svc.answer(
                Some(session.id),
                "fy1",
                &q.id,
                &Response::Choice {
                    index: right(&q.id),
                },
                60,
            )?;
            assert!(v.correct);
        }
        let result = svc.finish_session(session.id)?;
        assert_eq!((result.best, result.new_best), (Some(100), true));
        let again = svc.finish_session(session.id)?;
        assert!(!again.new_best, "finishing twice is no new best");

        let o = svc.overview(60)?;
        let fy1 = &o.subjects[0];
        assert_eq!(
            fy1.progress,
            Progress {
                questions: 2,
                seen: 2,
                correct: 2,
                wrong: 0,
                due: 0
            }
        );
        assert_eq!(o.answered_today, 2);
        assert_eq!(o.streak, 1);
        let quiz_set = fy1
            .sets
            .iter()
            .find(|s| s.key == "quiz:mech")
            .expect("quiz set");
        assert_eq!(quiz_set.best, Some(100));
        assert_eq!(fy1.chapters[0].questions, 2);
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
        assert!(!s.scored);
        let rows = svc.questions()?;
        let m2 = rows.iter().find(|r| r.id == "M2").expect("M2");
        assert_eq!(
            (m2.answered, m2.correct, m2.last_correct),
            (1, 0, Some(false))
        );
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

    #[test]
    fn activity_fills_every_day() -> Result<()> {
        let svc = service();
        svc.answer(None, "fy1", "M1", &Response::Choice { index: 0 }, 0)?;
        let a = svc.activity(7, 0)?;
        assert_eq!(a.len(), 7);
        assert_eq!(a.last().map(|d| d.answered), Some(1));
        assert!(a[..6].iter().all(|d| d.answered == 0));
        Ok(())
    }

    #[test]
    fn streaks_count_back_from_today_or_yesterday() {
        let d = |day, answered| DayActivity {
            day,
            answered,
            correct: 0,
        };
        assert_eq!(streak(&[d(8, 1), d(9, 2), d(10, 1)], 10), 3);
        assert_eq!(streak(&[d(8, 1), d(9, 2)], 10), 2, "today not started yet");
        assert_eq!(streak(&[d(7, 1), d(9, 2)], 10), 1);
        assert_eq!(streak(&[d(5, 1)], 10), 0);
    }
}
