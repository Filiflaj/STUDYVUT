//! `submission.toml` as written, before checking. Unknown keys are errors,
//! so a typo (`submited`) does not silently drop a field.

use serde::Deserialize;

use super::model::{Kind, Role};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSubmission {
    pub title: String,
    pub kind: Kind,
    /// Text or a whole number.
    pub task: Option<toml::Value>,
    /// A TOML date, or the same as a string.
    pub measured: Option<toml::Value>,
    pub submitted: Option<toml::Value>,
    pub summary: Option<String>,
    #[serde(default)]
    pub chapters: Vec<String>,
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default, rename = "file")]
    pub files: Vec<RawFile>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawFile {
    pub path: String,
    pub role: Role,
    pub label: Option<String>,
}
