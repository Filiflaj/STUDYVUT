//! Reading `submissions/` into submissions, checking as it goes.

use std::collections::{BTreeMap, BTreeSet};

use super::model::{Role, Submission, SubmissionFile};
use super::raw::{RawFile, RawSubmission};
use super::{MAX_SIZE, WARN_SIZE, accepted, valid_id, valid_iso_date};
use crate::content::markdown::dollars_balanced;
use crate::content::{Catalog, Files, Problem, Severity, Subject};

/// The metadata file of every submission folder.
pub const METADATA: &str = "submission.toml";

/// Everything here is published: a file the app does not show is an error.
const UNSHOWN: &str = "would be published and built into the app without being shown; \
                       list it in submission.toml or remove it";

/// Read every `<subject>/<id>/submission.toml` of `files` (the
/// `submissions/` folder) and the files it lists. `catalog` is the loaded
/// content: the subject must exist, chapters and topics are checked
/// against it. Returns the submissions (by subject, newest first) and
/// everything wrong, with paths relative to `files`.
pub fn load(files: &dyn Files, catalog: &Catalog) -> (Vec<Submission>, Vec<Problem>) {
    let paths = files.list();
    let mut folders: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
    let mut problems = Vec::new();
    for path in &paths {
        let mut parts = path.splitn(3, '/');
        match (parts.next(), parts.next(), parts.next()) {
            (Some(subject), Some(id), Some(rest)) => {
                folders.entry((subject, id)).or_default().insert(rest);
            }
            (Some(name), None, _) if name.ends_with(".md") => {} // a README at the top
            _ => problems.push(error(
                path,
                format!("outside any submission folder (<subject>/<id>/): {UNSHOWN}"),
            )),
        }
    }

    let mut out = Vec::new();
    let mut ctx = Ctx {
        files,
        catalog,
        problems: &mut problems,
    };
    for ((subject, id), rel) in &folders {
        if let Some(s) = ctx.submission(subject, id, rel) {
            out.push(s);
        }
    }
    out.sort_by(|a, b| {
        a.subject
            .cmp(&b.subject)
            .then_with(|| b.submitted.cmp(&a.submitted))
            .then_with(|| a.id.cmp(&b.id))
    });
    (out, problems)
}

fn error(path: &str, message: impl Into<String>) -> Problem {
    Problem {
        severity: Severity::Error,
        path: path.to_owned(),
        message: message.into(),
    }
}

fn warning(path: &str, message: impl Into<String>) -> Problem {
    Problem {
        severity: Severity::Warning,
        path: path.to_owned(),
        message: message.into(),
    }
}

/// The first line of a TOML error, which is where and what.
fn toml_message(e: &toml::de::Error) -> String {
    let msg = e.message().trim().to_owned();
    match e.span() {
        Some(span) => format!("{msg} (at byte {})", span.start),
        None => msg,
    }
}

/// Relative, inside the folder, no hidden or `.`/`..` parts, `/` only.
fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':'])
        && path
            .split('/')
            .all(|part| !part.is_empty() && !part.starts_with('.'))
}

/// Names that survive git, zip files, Windows and macOS (no spaces, no accents).
fn portable(path: &str) -> bool {
    path.chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '/'))
}

fn trimmed(text: Option<String>) -> Option<String> {
    text.map(|t| t.trim().to_owned()).filter(|t| !t.is_empty())
}

fn mb(bytes: u64) -> String {
    format!("{:.1} MB", bytes as f64 / f64::from(1 << 20))
}

struct Ctx<'a> {
    files: &'a dyn Files,
    catalog: &'a Catalog,
    problems: &'a mut Vec<Problem>,
}

impl Ctx<'_> {
    fn submission(&mut self, subject: &str, id: &str, rel: &BTreeSet<&str>) -> Option<Submission> {
        let folder = format!("{subject}/{id}");
        let at = format!("{folder}/{METADATA}");
        if !valid_id(subject) || !valid_id(id) {
            self.problems.push(error(
                &at,
                format!("folder {folder:?}: both parts must be lower case letters, digits and -"),
            ));
            return None;
        }
        if !rel.contains(METADATA) {
            self.problems.push(error(
                &format!("{folder}/"),
                format!("no {METADATA}: its files {UNSHOWN}"),
            ));
            return None;
        }
        let Some(text) = self.files.read_string(&at) else {
            self.problems
                .push(error(&at, "cannot be read as UTF-8 text"));
            return None;
        };
        if let Ok(table) = text.parse::<toml::Table>()
            && (table.contains_key("visibility") || table.contains_key("private"))
        {
            self.problems.push(error(
                &at,
                "everything in submissions/ is public (repository and app); \
                 work that is not to be published stays out of the repository",
            ));
            return None;
        }
        let raw: RawSubmission = match toml::from_str(&text) {
            Ok(r) => r,
            Err(e) => {
                self.problems.push(error(&at, toml_message(&e)));
                return None;
            }
        };
        let catalog = self.catalog;
        let Some(subj) = catalog.subject(subject) else {
            self.problems.push(error(
                &at,
                format!("no subject {subject:?} in content/ (add content/{subject}/subject.toml)"),
            ));
            return None;
        };
        let title = raw.title.trim().to_owned();
        if title.is_empty() {
            self.problems.push(error(&at, "title is empty"));
            return None;
        }

        let task = raw.task.and_then(|v| match v {
            toml::Value::String(s) => trimmed(Some(s)),
            toml::Value::Integer(i) => Some(i.to_string()),
            _ => {
                self.problems
                    .push(error(&at, "task must be text or a whole number"));
                None
            }
        });
        let measured = self.date(&at, "measured", raw.measured);
        let submitted = self.date(&at, "submitted", raw.submitted);
        if let (Some(m), Some(s)) = (&measured, &submitted)
            && s < m
        {
            self.problems.push(warning(
                &at,
                format!("submitted {s} is before measured {m}"),
            ));
        }
        let summary = trimmed(raw.summary);
        if summary.as_deref().is_some_and(|s| !dollars_balanced(s)) {
            self.problems.push(error(&at, "unpaired $ in the summary"));
        }
        let chapters = self.chapters(&at, subj, raw.chapters);
        let topics = self.topics(&at, subj, raw.topics);

        let mut seen = BTreeSet::new();
        let files: Vec<SubmissionFile> = raw
            .files
            .into_iter()
            .filter_map(|f| self.file(&folder, &at, f, rel, &mut seen))
            .collect();
        if files.is_empty() {
            self.problems.push(warning(&at, "no files"));
        } else if !files.iter().any(|f| f.role == Role::Final) {
            self.problems.push(warning(
                &at,
                "no file with role = \"final\" (the submitted document)",
            ));
        }
        for r in rel {
            if *r != METADATA && !seen.contains(*r) {
                self.problems.push(error(
                    &format!("{folder}/{r}"),
                    format!("not listed: it {UNSHOWN}"),
                ));
            }
        }

        Some(Submission {
            subject: subject.to_owned(),
            id: id.to_owned(),
            title,
            kind: raw.kind,
            task,
            measured,
            submitted,
            summary,
            chapters,
            topics,
            files,
        })
    }

    /// A TOML date (`2026-10-08`) or the same as a string; anything else
    /// is reported and left out.
    fn date(&mut self, at: &str, field: &str, value: Option<toml::Value>) -> Option<String> {
        let text = match value? {
            toml::Value::Datetime(d) if d.time.is_none() && d.offset.is_none() => d.to_string(),
            toml::Value::String(s) => s.trim().to_owned(),
            _ => String::new(),
        };
        if valid_iso_date(&text) {
            Some(text)
        } else {
            self.problems.push(error(
                at,
                format!("{field} must be a date like 2026-10-08 (left out)"),
            ));
            None
        }
    }

    /// Chapter ids of the subject; unknown ones are reported and left out.
    fn chapters(&mut self, at: &str, subject: &Subject, chapters: Vec<String>) -> Vec<String> {
        chapters
            .into_iter()
            .filter(|c| {
                subject.chapter(c).is_some() || {
                    self.problems.push(warning(
                        at,
                        format!("no chapter {c:?} in {} (link left out)", subject.id),
                    ));
                    false
                }
            })
            .collect()
    }

    /// Quiz topics; one no question has yet is kept but reported.
    fn topics(&mut self, at: &str, subject: &Subject, topics: Vec<String>) -> Vec<String> {
        let known = subject.topics();
        let topics: Vec<String> = topics
            .into_iter()
            .map(|t| t.trim().to_owned())
            .filter(|t| !t.is_empty())
            .collect();
        for t in &topics {
            if !known.contains(&t.as_str()) {
                self.problems
                    .push(warning(at, format!("topic {t:?} has no questions")));
            }
        }
        topics
    }

    /// A listed file: it exists, its name and type are fine, its size is sane.
    /// `seen` collects every listed path (also of rejected entries, which
    /// are reported once, here).
    fn file(
        &mut self,
        folder: &str,
        at: &str,
        raw: RawFile,
        rel: &BTreeSet<&str>,
        seen: &mut BTreeSet<String>,
    ) -> Option<SubmissionFile> {
        let path = raw.path.trim().to_owned();
        let label = format!("file {path:?}");
        let fail = |problems: &mut Vec<Problem>, msg: &str| {
            problems.push(error(at, format!("{label}: {msg}")));
        };
        if !safe_path(&path) {
            fail(
                self.problems,
                "must be a path inside the submission folder, with / and no hidden, . or .. parts",
            );
            return None;
        }
        if path == METADATA {
            fail(self.problems, "is the metadata itself");
            return None;
        }
        if !seen.insert(path.clone()) {
            fail(self.problems, "listed twice");
            return None;
        }
        if !rel.contains(path.as_str()) {
            fail(self.problems, "does not exist");
            return None;
        }
        if !accepted(&path) {
            fail(
                self.problems,
                "not a type a submission may hold (documents, spreadsheets, images, text, sources)",
            );
            return None;
        }
        if !portable(&path) {
            self.problems.push(warning(
                at,
                format!(
                    "{label}: use only a-z, 0-9, -, _ and . in file names \
                     (spaces and accents break in git, zip files and on other systems)"
                ),
            ));
        }
        let size = self
            .files
            .size(&format!("{folder}/{path}"))
            .unwrap_or_default();
        if size > MAX_SIZE {
            fail(
                self.problems,
                &format!(
                    "{} is over the {} limit (GitHub refuses large files)",
                    mb(size),
                    mb(MAX_SIZE)
                ),
            );
        } else if size > WARN_SIZE {
            self.problems.push(warning(
                at,
                format!(
                    "{label}: {} is large; every copy of the app carries it",
                    mb(size)
                ),
            ));
        }
        Some(SubmissionFile {
            path,
            role: raw.role,
            label: trimmed(raw.label),
            size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{self, MemFiles};

    const QUIZ: &str = r#"
name = "Mechanika"
[[question]]
id = "M1"
topic = "Tření"
text = "x"
options = ["a", "b"]
answer = 0
"#;

    fn catalog() -> Catalog {
        let files = MemFiles::new()
            .with(
                "bpc-fy1/subject.toml",
                "code = \"BPC-FY1\"\nname = \"Fyzika 1\"\nsemester = \"ZS 2026/2027\"",
            )
            .with("bpc-fy1/quizzes/mech.toml", QUIZ)
            .with(
                "bpc-fy1/chapters/04-treni.md",
                "+++\ntopics = [\"Tření\"]\n+++\n# Tření\n",
            );
        content::load(&files).0
    }

    const LAB: &str = r#"
title = "Rovnoměrně zrychlený pohyb po nakloněné rovině"
kind = "protocol"
task = 5
measured = 2026-10-15
submitted = 2026-10-21
summary = 'Součinitel tření z $a = g(\sin\alpha - \mu\cos\alpha)$.'
chapters = ["04-treni"]
topics = ["Tření"]

[[file]]
path = "protokol.pdf"
role = "final"

[[file]]
path = "data/mereni.xlsx"
role = "data"
label = "Měřená data"
"#;

    fn submissions() -> MemFiles {
        MemFiles::new()
            .with("README.md", "# Odevzdané práce")
            .with("bpc-fy1/lab-05/submission.toml", LAB)
            .with("bpc-fy1/lab-05/protokol.pdf", "%PDF-1.7")
            .with("bpc-fy1/lab-05/data/mereni.xlsx", "PK")
            .with("bpc-fy1/lab-05/.DS_Store", "")
    }

    fn messages(files: &MemFiles) -> Vec<String> {
        load(files, &catalog())
            .1
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    }

    #[test]
    fn a_valid_submission_loads_without_problems() {
        let (subs, problems) = load(&submissions(), &catalog());
        assert_eq!(problems, vec![]);
        let s = &subs[0];
        assert_eq!(s.folder(), "bpc-fy1/lab-05");
        assert_eq!(s.task.as_deref(), Some("5"), "a number reads as text");
        assert_eq!(s.measured.as_deref(), Some("2026-10-15"));
        assert_eq!(s.submitted.as_deref(), Some("2026-10-21"));
        assert_eq!(s.chapters, vec!["04-treni"]);
        assert_eq!(s.files.len(), 2);
        assert_eq!(s.files[0].size, 8);
        assert_eq!(s.size(), 10);
        assert_eq!(s.file_path(&s.files[1]), "bpc-fy1/lab-05/data/mereni.xlsx");
        assert_eq!(s.files[1].label.as_deref(), Some("Měřená data"));
    }

    #[test]
    fn nothing_is_published_without_being_shown() {
        let files = submissions()
            .with("bpc-fy1/lab-05/navic.pdf", "%PDF")
            .with("bpc-fy1/lab-06/protokol.pdf", "%PDF")
            .with("bpc-fy1/poznamky.txt", "x")
            .with("volne.pdf", "%PDF");
        let (subs, _) = load(&files, &catalog());
        assert_eq!(subs.len(), 1);
        let m = messages(&files);
        assert_eq!(m.len(), 4, "{m:?}");
        assert!(
            m.iter()
                .all(|x| x.starts_with("error:") && x.contains("without being shown")),
            "{m:?}"
        );
    }

    #[test]
    fn private_does_not_exist_in_a_public_folder() {
        let lab = LAB.replacen(
            "kind = \"protocol\"",
            "kind = \"protocol\"\nvisibility = \"private\"",
            1,
        );
        let files = submissions().with("bpc-fy1/lab-05/submission.toml", lab);
        let (subs, _) = load(&files, &catalog());
        assert!(subs.is_empty());
        let m = messages(&files);
        assert!(
            m.iter()
                .any(|x| x.contains("everything in submissions/ is public")),
            "{m:?}"
        );
    }

    #[test]
    fn broken_files_are_reported_and_left_out() {
        let lab = r#"
title = "Lab"
kind = "protocol"
submitted = "21. 10. 2026"
chapters = ["99-nic"]
topics = ["Kmity"]

[[file]]
path = "../tajne.pdf"
role = "final"

[[file]]
path = "chybi.pdf"
role = "final"

[[file]]
path = "Protokol hotový.pdf"
role = "source"

[[file]]
path = "Protokol hotový.pdf"
role = "data"

[[file]]
path = "spust.sh"
role = "source"
"#;
        let files = MemFiles::new()
            .with("bpc-fy1/lab-10/submission.toml", lab)
            .with("bpc-fy1/lab-10/Protokol hotový.pdf", "%PDF")
            .with("bpc-fy1/lab-10/spust.sh", "rm -rf ~");
        let (subs, _) = load(&files, &catalog());
        assert_eq!(subs.len(), 1, "the submission still loads");
        assert_eq!(subs[0].files.len(), 1);
        assert_eq!(subs[0].submitted, None);
        assert!(subs[0].chapters.is_empty());
        let m = messages(&files);
        let has = |needle: &str| m.iter().any(|x| x.contains(needle));
        assert!(has("submitted must be a date like 2026-10-08"), "{m:?}");
        assert!(has("no chapter \"99-nic\" in bpc-fy1"), "{m:?}");
        assert!(has("topic \"Kmity\" has no questions"), "{m:?}");
        assert!(has("file \"../tajne.pdf\": must be a path inside"), "{m:?}");
        assert!(has("file \"chybi.pdf\": does not exist"), "{m:?}");
        assert!(has("file \"Protokol hotový.pdf\": use only a-z"), "{m:?}");
        assert!(has("file \"Protokol hotový.pdf\": listed twice"), "{m:?}");
        assert!(has("file \"spust.sh\": not a type"), "{m:?}");
        assert!(has("no file with role = \"final\""), "{m:?}");
        assert!(
            !has("spust.sh: not listed"),
            "reported once, as a bad type: {m:?}"
        );
    }

    #[test]
    fn structural_mistakes_drop_the_submission() {
        let ok = "title = \"x\"\nkind = \"protocol\"";
        let files = MemFiles::new()
            .with("bpc-fy1/Lab 10/submission.toml", ok)
            .with(
                "bpc-fy1/lab-11/submission.toml",
                "title = \"x\"\nkind = \"protokol\"",
            )
            .with(
                "bpc-fy1/lab-12/submission.toml",
                "title = \" \"\nkind = \"protocol\"",
            )
            .with(
                "bpc-fy1/lab-13/submission.toml",
                "title = \"x\"\nkind = \"protocol\"\nsubmited = 2026-10-01",
            )
            .with("bpc-xyz/du-1/submission.toml", ok);
        let (subs, _) = load(&files, &catalog());
        assert!(subs.is_empty());
        let m = messages(&files);
        let has = |needle: &str| m.iter().any(|x| x.contains(needle));
        assert!(
            has("folder \"bpc-fy1/Lab 10\": both parts must be lower case"),
            "{m:?}"
        );
        assert!(has("unknown variant `protokol`"), "{m:?}");
        assert!(has("lab-12/submission.toml: title is empty"), "{m:?}");
        assert!(has("unknown field `submited`"), "{m:?}");
        assert!(has("no subject \"bpc-xyz\" in content/"), "{m:?}");
    }

    #[test]
    fn sizes_are_watched() {
        let big = vec![0u8; usize::try_from(WARN_SIZE).unwrap_or(0) + 1];
        let files = submissions().with("bpc-fy1/lab-05/protokol.pdf", big);
        let m = messages(&files);
        assert!(
            m.len() == 1 && m[0].starts_with("warning:") && m[0].contains("is large"),
            "{m:?}"
        );
    }

    #[test]
    fn newest_first_within_a_subject() {
        let s = |date: &str| {
            format!(
                "title = \"x\"\nkind = \"protocol\"\nsubmitted = {date}\n[[file]]\npath = \"p.pdf\"\nrole = \"final\""
            )
        };
        let files = MemFiles::new()
            .with("bpc-fy1/lab-a/submission.toml", s("2026-10-01"))
            .with("bpc-fy1/lab-a/p.pdf", "")
            .with("bpc-fy1/lab-b/submission.toml", s("2026-11-01"))
            .with("bpc-fy1/lab-b/p.pdf", "")
            .with(
                "bpc-fy1/lab-c/submission.toml",
                "title = \"x\"\nkind = \"other\"\n[[file]]\npath = \"p.pdf\"\nrole = \"final\"",
            )
            .with("bpc-fy1/lab-c/p.pdf", "");
        let (subs, _) = load(&files, &catalog());
        let ids: Vec<_> = subs.iter().map(|s| s.id.as_str()).collect();
        assert_eq!(ids, vec!["lab-b", "lab-a", "lab-c"], "undated last");
    }
}
