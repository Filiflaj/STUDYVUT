//! The files as written, before checking. Unknown keys are errors, so a
//! typo (`expalnation`) does not silently drop a field.

use serde::Deserialize;

use super::model::{Color, Pick};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSubject {
    pub code: String,
    pub name: String,
    pub semester: String,
    pub year: Option<u8>,
    #[serde(default)]
    pub programs: Vec<String>,
    #[serde(default)]
    pub color: Color,
    pub description: Option<String>,
    #[serde(default, rename = "set")]
    pub sets: Vec<RawSet>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawPrograms {
    #[serde(default, rename = "program")]
    pub programs: Vec<RawProgram>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawProgram {
    pub id: String,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawSet {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub pick: Option<Vec<Pick>>,
    pub shuffle: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawQuiz {
    pub name: String,
    pub description: Option<String>,
    #[serde(default, rename = "question")]
    pub questions: Vec<toml::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawQuestion {
    pub id: String,
    #[serde(default)]
    pub kind: RawKind,
    pub topic: String,
    pub source: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub text: String,
    pub figure: Option<String>,
    #[serde(default)]
    pub explanation: String,
    pub warning: Option<String>,
    // choice
    pub options: Option<Vec<String>>,
    pub answer: Option<usize>,
    pub notes: Option<Vec<String>>,
    pub layout: Option<RawLayout>,
    // numeric
    pub value: Option<f64>,
    pub tolerance: Option<f64>,
    pub unit: Option<String>,
    pub sign_hint: Option<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawKind {
    #[default]
    Choice,
    Numeric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawLayout {
    List,
    Grid,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RawFrontMatter {
    pub title: Option<String>,
    #[serde(default)]
    pub topics: Vec<String>,
}
