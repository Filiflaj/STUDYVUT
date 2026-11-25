//! The study material: `content/<subject>/` read into a [`Catalog`].
//!
//! The format is described in `docs/content.md`. [`load`] reads what it can
//! and reports everything else as a [`Problem`]: a broken question is left
//! out, the rest of its subject still loads. `studyvut-cli check` fails on
//! any [`Severity::Error`]; the app shows the count in its settings.

mod files;
mod load;
pub mod markdown;
mod model;
mod raw;

pub use files::{DirFiles, Files, MemFiles};
pub use load::load;
pub use model::{
    Catalog, Chapter, Color, Document, Pick, Program, Question, QuestionKind, Quiz, SetDef,
    SetRule, Subject,
};

use serde::Serialize;

/// Something wrong with the content, and where.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Problem {
    pub severity: Severity,
    /// The file, relative to the content folder (`bpc-fy1/quizzes/mechanika.toml`).
    pub path: String,
    /// What is wrong, in English, starting with the item when there is one
    /// (`question M3: answer 5 is out of range`).
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    /// Left out or wrong in the app.
    Error,
    /// Works, but probably not as meant.
    Warning,
}

impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        write!(f, "{level}: {}: {}", self.path, self.message)
    }
}

/// The media type of a content file, by its extension.
pub fn mime_type(path: &str) -> &'static str {
    match path
        .rsplit('.')
        .next()
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("md") => "text/markdown; charset=utf-8",
        Some("toml") => "application/toml",
        _ => "application/octet-stream",
    }
}

/// Image files the content may refer to.
pub fn is_image(path: &str) -> bool {
    mime_type(path).starts_with("image/")
}
