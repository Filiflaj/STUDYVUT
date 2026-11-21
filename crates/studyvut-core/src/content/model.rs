//! The loaded content, as the app and the CLI use it.

use serde::{Deserialize, Serialize};

/// Every subject, newest semester first, then by code.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Catalog {
    /// Study programmes (`programs.toml`), in file order.
    pub programs: Vec<Program>,
    pub subjects: Vec<Subject>,
}

/// A study programme (obor), e.g. BPC-MET.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Program {
    /// Lower case, e.g. `met`; subjects refer to it.
    pub id: String,
    pub code: String,
    pub name: String,
}

impl Catalog {
    pub fn subject(&self, id: &str) -> Option<&Subject> {
        self.subjects.iter().find(|s| s.id == id)
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Subject {
    /// The folder name, e.g. `bpc-fy1`.
    pub id: String,
    pub code: String,
    pub name: String,
    pub semester: String,
    /// The study year (ročník) the subject is taught in.
    pub year: Option<u8>,
    /// The programmes (ids) that have it; empty for every programme.
    pub programs: Vec<String>,
    pub color: Color,
    pub description: Option<String>,
    /// The chapter on how the subject is assessed (`info` in subject.toml).
    pub info: Option<String>,
    pub chapters: Vec<Chapter>,
    pub quizzes: Vec<Quiz>,
    /// The custom sets of `subject.toml`, in file order.
    pub sets: Vec<SetDef>,
}

impl Subject {
    pub fn questions(&self) -> impl Iterator<Item = &Question> {
        self.quizzes.iter().flat_map(|q| q.questions.iter())
    }

    pub fn question(&self, id: &str) -> Option<&Question> {
        self.questions().find(|q| q.id == id)
    }

    pub fn quiz(&self, id: &str) -> Option<&Quiz> {
        self.quizzes.iter().find(|q| q.id == id)
    }

    pub fn chapter(&self, id: &str) -> Option<&Chapter> {
        self.chapters.iter().find(|c| c.id == id)
    }

    pub fn set(&self, id: &str) -> Option<&SetDef> {
        self.sets.iter().find(|s| s.id == id)
    }

    /// Every topic of the subject's questions, in order of first use.
    pub fn topics(&self) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for q in self.questions() {
            if !out.contains(&q.topic.as_str()) {
                out.push(&q.topic);
            }
        }
        out
    }
}

/// The subject's colour, from the app's palette (checked for contrast and
/// colour blindness).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    #[default]
    Blue,
    Green,
    Pink,
    Gold,
    Violet,
    Teal,
    Orange,
    Graphite,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Chapter {
    /// The file name without `.md`.
    pub id: String,
    pub title: String,
    /// Quiz topics practised from this chapter.
    pub topics: Vec<String>,
    /// The text without its front matter.
    #[serde(skip_serializing)]
    pub markdown: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Quiz {
    /// The file name without `.toml`.
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub questions: Vec<Question>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Question {
    pub id: String,
    pub topic: String,
    pub source: Option<String>,
    pub tags: Vec<String>,
    /// Markdown.
    pub text: String,
    /// An image path relative to the subject folder.
    pub figure: Option<String>,
    pub explanation: String,
    pub warning: Option<String>,
    #[serde(flatten)]
    pub kind: QuestionKind,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum QuestionKind {
    Choice {
        options: Vec<String>,
        /// Index of the right option.
        answer: usize,
        /// Per option, why it is wrong ("" when there is nothing to say).
        notes: Option<Vec<String>>,
        /// Options in two columns.
        grid: bool,
    },
    Numeric {
        value: f64,
        tolerance: f64,
        unit: String,
        /// Shown when the answer is right except for its sign.
        sign_hint: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SetDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub rule: SetRule,
    pub shuffle: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum SetRule {
    /// Every question with any of these tags.
    Tags(Vec<String>),
    /// Random questions from quiz files.
    Pick(Vec<Pick>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Pick {
    pub quiz: String,
    pub count: usize,
}
