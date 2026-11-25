//! Copies of embedded files in the cache folder, so that the system can
//! open them (a PDF viewer needs a real file on the disk).

use std::path::{Path, PathBuf};

/// `bytes` as the file `path`, written when missing or different (a newer
/// build may carry a corrected file); written aside first, then renamed.
pub(crate) fn copy(path: &Path, bytes: &[u8]) -> std::io::Result<PathBuf> {
    if std::fs::read(path).ok().as_deref() == Some(bytes) {
        return Ok(path.to_path_buf());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(format!("part-{}", std::process::id()));
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)?;
    Ok(path.to_path_buf())
}
