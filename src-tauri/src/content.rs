//! Where the app reads its content from.
//!
//! - `STUDYVUT_CONTENT_DIR`, when set: that folder;
//! - debug builds: the repository's `content/` folder, read anew on
//!   **Reload content**, so edits show without a rebuild;
//! - release builds: the copy embedded at build time (`build.rs` rebuilds
//!   when the folder changes).
//!
//! The submitted work (`submissions/`, docs/submissions.md) comes the same
//! way, with `STUDYVUT_SUBMISSIONS_DIR`.

use std::path::PathBuf;
use std::sync::Arc;

use include_dir::{Dir, DirEntry, include_dir};
use studyvut_core::content::{DirFiles, Files};
use studyvut_service::ContentSource;

static EMBEDDED: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../content");
static EMBEDDED_SUBMISSIONS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../submissions");

/// The submitted work, from where the content comes from.
pub fn submissions() -> Arc<dyn Files> {
    let folder = std::env::var_os("STUDYVUT_SUBMISSIONS_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            cfg!(debug_assertions)
                .then(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../submissions"))
                .filter(|p| p.is_dir())
        });
    match folder {
        Some(path) => Arc::new(DirFiles::new(path.canonicalize().unwrap_or(path))),
        None => Arc::new(Embedded(&EMBEDDED_SUBMISSIONS)),
    }
}

pub fn source() -> (Arc<dyn Files>, ContentSource) {
    let folder = std::env::var_os("STUDYVUT_CONTENT_DIR")
        .map(PathBuf::from)
        .or_else(|| {
            cfg!(debug_assertions)
                .then(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../content"))
                .filter(|p| p.is_dir())
        });
    match folder {
        Some(path) => {
            let path = path.canonicalize().unwrap_or(path);
            (
                Arc::new(DirFiles::new(&path)),
                ContentSource::Folder { path },
            )
        }
        None => (Arc::new(Embedded(&EMBEDDED)), ContentSource::Embedded),
    }
}

/// A folder built into the app (the content, the submitted work).
struct Embedded(&'static Dir<'static>);

impl Files for Embedded {
    fn list(&self) -> Vec<String> {
        let mut out = Vec::new();
        walk(self.0, &mut out);
        out.sort();
        out
    }

    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.0.get_file(path).map(|f| f.contents().to_vec())
    }

    fn size(&self, path: &str) -> Option<u64> {
        self.0.get_file(path).map(|f| f.contents().len() as u64)
    }
}

fn walk(dir: &Dir<'_>, out: &mut Vec<String>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(d) => walk(d, out),
            DirEntry::File(f) => {
                let path = f.path().to_string_lossy().replace('\\', "/");
                let hidden = path.rsplit('/').next().is_some_and(|n| n.starts_with('.'));
                if !hidden {
                    out.push(path);
                }
            }
        }
    }
}
