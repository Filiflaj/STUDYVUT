//! Persistent user settings (`settings.json` in the data folder).

use std::path::Path;

use serde::{Deserialize, Serialize};
use tracing::warn;

/// Raised whenever a change needs an upgrade step in [`Settings::parse`].
pub const SETTINGS_VERSION: u32 = 1;

/// Colour scheme of the window.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThemePref {
    #[default]
    System,
    Light,
    Dark,
}

/// User interface language. Czech by default, since the content is Czech
/// (an English system would otherwise mix an English UI with Czech texts).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguagePref {
    System,
    #[default]
    Cs,
    En,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct Settings {
    pub version: u32,
    pub theme: ThemePref,
    pub language: LanguagePref,
    /// Answers a day the Dashboard counts towards.
    pub daily_goal: u32,
    /// Subjects left out of the sidebar and the Dashboard (not studied).
    pub hidden_subjects: Vec<String>,
    /// The app version whose "Co je nového" the user has seen; None on a
    /// fresh install until the first start stores it.
    pub last_seen_version: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            version: SETTINGS_VERSION,
            theme: ThemePref::System,
            language: LanguagePref::Cs,
            daily_goal: 20,
            hidden_subjects: Vec::new(),
            last_seen_version: None,
        }
    }
}

enum Parsed {
    Complete(Settings),
    /// Valid JSON, but some values did not fit and read as defaults.
    Partial(Settings, String),
    NotJson(String),
}

impl Settings {
    /// Values within their ranges.
    pub fn validated(mut self) -> Self {
        self.version = SETTINGS_VERSION;
        self.daily_goal = self.daily_goal.clamp(1, 500);
        self.hidden_subjects.sort();
        self.hidden_subjects.dedup();
        self
    }

    /// Read the settings; `true` when there was no file (first run). A
    /// damaged file is kept next to it as `settings.json.bad` and what could
    /// be read is used.
    pub fn load(path: &Path) -> (Self, bool) {
        let text = match std::fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return (Self::default().validated(), true);
            }
            Err(e) => {
                warn!("cannot read the settings ({e}); using defaults");
                return (Self::default().validated(), false);
            }
        };
        match Self::parse(&text) {
            Parsed::Complete(s) => (s, false),
            Parsed::Partial(s, e) => {
                warn!("some settings are invalid ({e}); keeping the valid ones");
                backup(path);
                (s, false)
            }
            Parsed::NotJson(e) => {
                warn!("the settings file is not valid JSON ({e}); using defaults");
                backup(path);
                (Self::default().validated(), false)
            }
        }
    }

    fn parse(text: &str) -> Parsed {
        // Windows editors may save a byte order mark, which is no JSON.
        let text = text.strip_prefix('\u{feff}').unwrap_or(text);
        let v = match serde_json::from_str::<serde_json::Value>(text) {
            Ok(v) => v,
            Err(e) => return Parsed::NotJson(e.to_string()),
        };
        match serde_json::from_value::<Settings>(v.clone()) {
            Ok(s) => Parsed::Complete(s.validated()),
            Err(e) => Parsed::Partial(lenient(&v).validated(), e.to_string()),
        }
    }

    /// Save atomically: write a temporary file, flush it, then rename it
    /// over the old one, so a crash never leaves half a file.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        use std::io::Write;
        static SAVES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = SAVES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let tmp = path.with_extension(format!("json.tmp-{}-{n}", std::process::id()));
        let json = serde_json::to_vec_pretty(self).map_err(std::io::Error::other)?;
        let mut file = std::fs::File::create(&tmp)?;
        file.write_all(&json)?;
        file.sync_all()?;
        drop(file);
        std::fs::rename(&tmp, path)
    }
}

/// Every field that reads on its own; the others keep their defaults.
fn lenient(v: &serde_json::Value) -> Settings {
    let mut s = Settings::default();
    let Some(obj) = v.as_object() else { return s };
    macro_rules! field {
        ($name:ident) => {
            if let Some(x) = obj
                .get(stringify!($name))
                .and_then(|x| serde_json::from_value(x.clone()).ok())
            {
                s.$name = x;
            }
        };
    }
    field!(theme);
    field!(language);
    field!(daily_goal);
    field!(hidden_subjects);
    field!(last_seen_version);
    s
}

fn backup(path: &Path) {
    let bad = path.with_extension("json.bad");
    if let Err(e) = std::fs::copy(path, &bad) {
        warn!("cannot keep the damaged settings as {}: {e}", bad.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_file_is_a_first_run() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let (s, first) = Settings::load(&dir.path().join("settings.json"));
        assert!(first);
        assert_eq!(s, Settings::default());
        Ok(())
    }

    #[test]
    fn settings_round_trip() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("settings.json");
        let s = Settings {
            theme: ThemePref::Dark,
            language: LanguagePref::En,
            daily_goal: 35,
            hidden_subjects: vec!["bpc-tde".into()],
            last_seen_version: Some("0.1.0".into()),
            ..Settings::default()
        };
        s.save(&path)?;
        assert_eq!(Settings::load(&path), (s, false));
        Ok(())
    }

    #[test]
    fn a_bad_value_keeps_the_others() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("settings.json");
        std::fs::write(
            &path,
            r#"{"theme":"dark","daily_goal":"lots","language":"cs"}"#,
        )?;
        let (s, first) = Settings::load(&path);
        assert!(!first);
        assert_eq!(
            (s.theme, s.language, s.daily_goal),
            (ThemePref::Dark, LanguagePref::Cs, 20)
        );
        assert!(dir.path().join("settings.json.bad").exists());

        std::fs::write(&path, "not json")?;
        assert_eq!(Settings::load(&path).0, Settings::default());
        Ok(())
    }

    #[test]
    fn values_are_kept_in_range() {
        let s = Settings {
            daily_goal: 0,
            hidden_subjects: vec!["b".into(), "a".into(), "b".into()],
            ..Settings::default()
        };
        let v = s.validated();
        assert_eq!(v.daily_goal, 1);
        assert_eq!(v.hidden_subjects, vec!["a", "b"]);
    }
}
