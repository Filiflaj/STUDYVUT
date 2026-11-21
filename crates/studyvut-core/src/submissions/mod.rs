//! The user's own submitted work (lab protocols, homework, projects), as
//! submitted: `submissions/<subject>/<id>/submission.toml` and the files it
//! lists, read into [`Submission`]s. The format is described in
//! `docs/submissions.md`.
//!
//! `submissions/` sits next to `content/`, is public like it (committed,
//! embedded in the app) and is checked against the loaded content: a
//! submission links chapters and quiz topics of its subject. [`load`]
//! reads what it can and reports the rest as
//! [`Problem`](crate::content::Problem)s; every file in the folder must be
//! listed, so nothing is published without being shown.

mod load;
mod model;
mod raw;

pub use load::{METADATA, load};
pub use model::{Kind, Role, Submission, SubmissionFile};

/// A file this large is worth a look (a scan at too high a resolution?).
pub const WARN_SIZE: u64 = 10 << 20;
/// GitHub warns from 50 MB a file and refuses 100 MB; the app embeds it all.
pub const MAX_SIZE: u64 = 50 << 20;

/// Opened in their system app: documents, spreadsheets, images, text.
const OPEN: &[&str] = &[
    "pdf", "docx", "doc", "xlsx", "xls", "pptx", "ppt", "odt", "ods", "odp", "csv", "txt", "md",
    "tex", "png", "jpg", "jpeg", "webp",
];

/// Sources that may be published but are only shown in their folder: the
/// system would run them instead of opening them.
const SOURCES: &[&str] = &[
    "c",
    "h",
    "cpp",
    "hpp",
    "py",
    "m",
    "ipynb",
    "json",
    "toml",
    "bib",
    "cls",
    "sty",
    "asc",
    "kicad_sch",
    "kicad_pcb",
    "kicad_pro",
];

/// The lower-case extension of a path (`""` when it has none).
fn extension(path: &str) -> String {
    let name = path.rsplit('/').next().unwrap_or(path);
    match name.rsplit_once('.') {
        Some((stem, ext)) if !stem.is_empty() => ext.to_ascii_lowercase(),
        _ => String::new(),
    }
}

/// Whether the app may hand a file to the system to open (documents, not
/// scripts or programs).
pub fn openable(path: &str) -> bool {
    OPEN.contains(&extension(path).as_str())
}

/// Whether a submission may contain a file of this type at all.
pub fn accepted(path: &str) -> bool {
    let ext = extension(path);
    OPEN.contains(&ext.as_str()) || SOURCES.contains(&ext.as_str())
}

/// Whether a text is a real `YYYY-MM-DD` date.
fn valid_iso_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return false;
    }
    let num = |r: std::ops::Range<usize>| s.get(r).and_then(|p| p.parse::<u32>().ok());
    let (Some(y), Some(m), Some(d)) = (num(0..4), num(5..7), num(8..10)) else {
        return false;
    };
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let days = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&d)
}

/// Lower case letters, digits and `-`: subject and submission folders.
fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_real_dates_are_valid() {
        assert!(valid_iso_date("2026-10-08"));
        assert!(valid_iso_date("2028-02-29"));
        assert!(!valid_iso_date("2026-02-29"));
        assert!(!valid_iso_date("2026-13-01"));
        assert!(!valid_iso_date("8. 10. 2026"));
        assert!(!valid_iso_date("2026-1-08"));
    }

    #[test]
    fn documents_open_and_programs_never_do() {
        assert!(openable("protokol.pdf"));
        assert!(openable("data/mereni.XLSX"));
        assert!(openable("data/smycka-50hz.xls"));
        for name in [
            "main.py",
            "run.sh",
            "setup.exe",
            "x.command",
            "Makefile",
            "a.pdf.app",
        ] {
            assert!(!openable(name), "{name}");
        }
        assert!(accepted("zdroj/main.py"), "a source may be published");
        for name in ["run.sh", "setup.exe", "prace.zip", "x.app", "README"] {
            assert!(!accepted(name), "{name}");
        }
    }
}
