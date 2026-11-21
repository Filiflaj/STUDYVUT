//! What the service reports to its front ends, ready to serialize.

use std::collections::BTreeMap;

use serde::Serialize;
use studyvut_core::content::{Color, Program, Question};
use studyvut_core::quiz::SetSpec;

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
    /// The study programmes, for the programme menu.
    pub programs: Vec<Program>,
    pub subjects: Vec<SubjectView>,
    pub problems: ProblemCount,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubjectView {
    pub id: String,
    pub code: String,
    pub name: String,
    pub semester: String,
    /// The study year (ročník); None when the content does not say.
    pub year: Option<u8>,
    /// The programmes that have it; empty for every programme.
    pub programs: Vec<String>,
    pub color: Color,
    pub description: Option<String>,
    pub chapters: Vec<ChapterView>,
    pub sets: Vec<SetView>,
    /// Questions in all its quizzes.
    pub questions: usize,
    /// Questions due for review today.
    pub due: usize,
    /// Submitted work of the subject (`submissions/`), for its tab.
    pub submissions: usize,
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
    pub questions: Vec<Question>,
    pub svgs: BTreeMap<String, String>,
}
