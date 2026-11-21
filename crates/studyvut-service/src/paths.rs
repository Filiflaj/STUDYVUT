//! Where settings, the progress database and logs live: one data folder,
//! shared by the desktop app and the CLI.
//!
//! In order:
//! - a folder given on the command line (the CLI's `--data-dir`);
//! - `STUDYVUT_DATA_DIR`;
//! - the platform's local data folder (the same as Tauri's
//!   `app_local_data_dir`): `~/Library/Application Support/<id>` on macOS,
//!   `%LOCALAPPDATA%\<id>` on Windows, `~/.local/share/<id>` on Linux.

use std::path::PathBuf;

use serde::Serialize;

/// Bundle identifier (must match `identifier` in tauri.conf.json).
pub const IDENTIFIER: &str = "io.github.filiflaj.studyvut";

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Paths {
    pub data_dir: PathBuf,
    /// Not the platform's folder (`--data-dir`, `STUDYVUT_DATA_DIR`): a
    /// development instance next to the real one.
    #[serde(skip)]
    pub custom: bool,
}

impl Paths {
    /// The data folder; `None` only when the platform has no local data folder.
    pub fn resolve(dir: Option<PathBuf>) -> Option<Self> {
        let given = dir.or_else(|| std::env::var_os("STUDYVUT_DATA_DIR").map(PathBuf::from));
        let custom = given.is_some();
        let data_dir = match given {
            Some(dir) => dir,
            None => dirs::data_local_dir()?.join(IDENTIFIER),
        };
        Some(Self { data_dir, custom })
    }

    pub fn create(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.data_dir)
    }

    pub fn settings_file(&self) -> PathBuf {
        self.data_dir.join("settings.json")
    }

    pub fn db_file(&self) -> PathBuf {
        self.data_dir.join("studyvut.db")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.data_dir.join("logs")
    }

    /// A WebView profile of its own for development instances (which would
    /// otherwise share the real one's local storage).
    pub fn webview_dir(&self) -> Option<PathBuf> {
        self.custom.then(|| self.data_dir.join("webview"))
    }
}
