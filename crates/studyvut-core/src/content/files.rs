//! Where the content is read from: a folder on the disk (the CLI, debug
//! builds of the app), the copy embedded in the app, or memory (tests).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A read-only tree of files. Paths are relative and use `/`.
pub trait Files: Send + Sync {
    /// Every file, sorted, without hidden ones (`.DS_Store`, `.gitkeep`).
    fn list(&self) -> Vec<String>;
    fn read(&self, path: &str) -> Option<Vec<u8>>;

    fn read_string(&self, path: &str) -> Option<String> {
        self.read(path).and_then(|b| String::from_utf8(b).ok())
    }

    /// The size in bytes, without reading the file where the source can.
    fn size(&self, path: &str) -> Option<u64> {
        self.read(path).map(|b| b.len() as u64)
    }
}

/// A folder on the disk, read anew on every call (so a reload sees edits).
pub struct DirFiles {
    root: PathBuf,
}

impl DirFiles {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

impl Files for DirFiles {
    fn list(&self) -> Vec<String> {
        let mut out = Vec::new();
        walk(&self.root, "", &mut out);
        out.sort();
        out
    }

    fn read(&self, path: &str) -> Option<Vec<u8>> {
        // Never outside the folder.
        if path.split('/').any(|p| p == ".." || p.is_empty()) {
            return None;
        }
        std::fs::read(self.root.join(path)).ok()
    }

    fn size(&self, path: &str) -> Option<u64> {
        if path.split('/').any(|p| p == ".." || p.is_empty()) {
            return None;
        }
        std::fs::metadata(self.root.join(path))
            .ok()
            .filter(|m| m.is_file())
            .map(|m| m.len())
    }
}

fn walk(dir: &Path, prefix: &str, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        match entry.file_type() {
            Ok(t) if t.is_dir() => walk(&entry.path(), &rel, out),
            Ok(t) if t.is_file() => out.push(rel),
            _ => {}
        }
    }
}

/// Files in memory, for tests and the embedded copy.
#[derive(Default)]
pub struct MemFiles {
    files: BTreeMap<String, Vec<u8>>,
}

impl MemFiles {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, path: &str, contents: impl Into<Vec<u8>>) -> Self {
        self.insert(path, contents);
        self
    }

    pub fn insert(&mut self, path: &str, contents: impl Into<Vec<u8>>) {
        self.files.insert(path.to_owned(), contents.into());
    }
}

impl Files for MemFiles {
    fn list(&self) -> Vec<String> {
        self.files
            .keys()
            .filter(|p| !p.rsplit('/').next().is_some_and(|n| n.starts_with('.')))
            .cloned()
            .collect()
    }

    fn read(&self, path: &str) -> Option<Vec<u8>> {
        self.files.get(path).cloned()
    }

    fn size(&self, path: &str) -> Option<u64> {
        self.files.get(path).map(|b| b.len() as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_folder_lists_its_files_without_hidden_ones() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir_all(dir.path().join("a/b"))?;
        std::fs::write(dir.path().join("a/b/x.md"), "x")?;
        std::fs::write(dir.path().join("a/.DS_Store"), "")?;
        std::fs::write(dir.path().join("top.toml"), "")?;
        let files = DirFiles::new(dir.path());
        assert_eq!(files.list(), vec!["a/b/x.md", "top.toml"]);
        assert_eq!(files.read_string("a/b/x.md").as_deref(), Some("x"));
        assert_eq!(files.read("../etc/passwd"), None);
        assert_eq!(files.read("a//b/x.md"), None);
        assert_eq!(files.size("a/b/x.md"), Some(1));
        assert_eq!(files.size("a/b"), None, "a folder has no size");
        assert_eq!(files.size("../a/b/x.md"), None);
        Ok(())
    }
}
