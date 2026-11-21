//! Submitted work as the app and the CLI use it.

use serde::{Deserialize, Serialize};

/// One submitted work: a folder `<subject>/<id>/` with its `submission.toml`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Submission {
    /// The subject folder, e.g. `bpc-fy1` (a subject of the content).
    pub subject: String,
    /// The submission folder, e.g. `lab-10`.
    pub id: String,
    pub title: String,
    pub kind: Kind,
    /// The task's number or code as the course names it (`10`, `L-1`, `DU3`).
    pub task: Option<String>,
    /// `YYYY-MM-DD`: when it was measured (labs).
    pub measured: Option<String>,
    /// `YYYY-MM-DD`.
    pub submitted: Option<String>,
    /// What the work was about and what to watch out for (Markdown).
    pub summary: Option<String>,
    /// Chapter ids of the subject.
    pub chapters: Vec<String>,
    /// Quiz topics of the subject, for "practise the topics of this work".
    pub topics: Vec<String>,
    pub files: Vec<SubmissionFile>,
}

impl Submission {
    /// `bpc-fy1/lab-10`: the folder within `submissions/`.
    pub fn folder(&self) -> String {
        format!("{}/{}", self.subject, self.id)
    }

    /// A file's path within `submissions/`, as [`Files`](crate::content::Files) reads it.
    pub fn file_path(&self, file: &SubmissionFile) -> String {
        format!("{}/{}", self.folder(), file.path)
    }

    /// Bytes of all its files.
    pub fn size(&self) -> u64 {
        self.files.iter().map(|f| f.size).sum()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    /// A lab protocol.
    Protocol,
    Homework,
    Project,
    /// A written test or exam that came back.
    Test,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmissionFile {
    /// Relative to the submission folder, with `/` (`protokol.pdf`, `data/mereni.xlsx`).
    pub path: String,
    pub role: Role,
    /// Shown instead of the file name, e.g. "Měřená data (Excel)".
    pub label: Option<String>,
    pub size: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// The document as submitted (usually the PDF).
    Final,
    /// What it was made from: LaTeX, Word, a spreadsheet with the calculations.
    Source,
    /// Measured or exported data.
    Data,
    Other,
}
