//! What the service reports to its front ends, ready to serialize.

use std::collections::BTreeMap;

use serde::Serialize;
use studyvut_core::content::{Color, Question};
use studyvut_core::quiz::SetSpec;
use studyvut_core::store::{DayActivity, SessionRow};

/// Counts of content problems, by severity.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ProblemCount {
    pub errors: usize,
    pub warnings: usize,
}

/// Everything the Dashboard and the sidebar show.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Overview {
    pub subjects: Vec<SubjectView>,
    pub problems: ProblemCount,
    /// The user's local day.
    pub today: i64,
    pub answered_today: u32,
    pub correct_today: u32,
    /// Days in a row with at least one answer, up to today (or yesterday,
    /// while today has none yet).
    pub streak: u32,
    pub recent: Vec<SessionRow>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubjectView {
    pub id: String,
    pub code: String,
    pub name: String,
    pub semester: String,
    pub color: Color,
    pub description: Option<String>,
    pub chapters: Vec<ChapterView>,
    pub sets: Vec<SetView>,
    pub progress: Progress,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChapterView {
    pub id: String,
    pub title: String,
    pub topics: Vec<String>,
    /// Questions practised from it.
    pub questions: usize,
}

/// A quiz set the user can start.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SetView {
    pub spec: SetSpec,
    /// `custom`, `quiz`, `mistakes` or `due`; the last two are named by the app.
    pub kind: &'static str,
    /// The key sessions store (`custom:old`, `quiz:mechanika`, …).
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    /// Questions it asks now.
    pub count: usize,
    /// Best finished score in percent.
    pub best: Option<u32>,
}

/// Where the user stands in a subject. Every question is in exactly one of
/// `correct` (latest answer right), `wrong` (latest wrong) or unseen.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Progress {
    pub questions: usize,
    pub seen: usize,
    pub correct: usize,
    pub wrong: usize,
    /// Due for review today.
    pub due: usize,
}

/// A chapter to read.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ChapterPayload {
    pub subject: String,
    pub id: String,
    pub title: String,
    pub topics: Vec<String>,
    pub markdown: String,
    /// The SVG images it refers to, by path, to inline.
    pub svgs: BTreeMap<String, String>,
    pub questions: usize,
}

/// A started session with its questions.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionPayload {
    pub id: i64,
    pub subject: String,
    pub key: String,
    pub scored: bool,
    pub questions: Vec<Question>,
    pub svgs: BTreeMap<String, String>,
}

/// A finished session and the set's best score.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SessionResult {
    pub session: SessionRow,
    pub best: Option<u32>,
    /// This session set a new best.
    pub new_best: bool,
}

/// A question of the content and the user's history with it, for the Data page.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionRow {
    pub subject: String,
    pub id: String,
    pub quiz: String,
    pub topic: String,
    /// Markdown.
    pub text: String,
    pub answered: u32,
    pub correct: u32,
    pub last_correct: Option<bool>,
    pub last_ts: Option<i64>,
    pub r#box: Option<u8>,
    pub due_day: Option<i64>,
}

/// Answers per local day, every day of the span (zero when none).
pub type Activity = Vec<DayActivity>;
