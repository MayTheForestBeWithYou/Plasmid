use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::walker::error::WalkerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMapping {
    pub source: PathBuf,
    pub dest: PathBuf,
}

/// Abstraction layer for recursively walking directories.
pub trait Walker {
    /// Walks the directory and resolves destination
    ///
    /// # Errors
    /// This method returns an error if:
    /// - the underlying filesystem call fails
    /// - the underyling walkdir call fails
    fn walk(
        &self,
        source_root: &Path,
        dest_root: &Path,
        ignore_patterns: &[String],
    ) -> Result<Vec<FileMapping>, WalkerError>;
}

pub struct WalkdirWalker;

impl WalkdirWalker {
    fn should_ignore(entry: &DirEntry, patterns: &[String]) -> bool {
        let Some(file_name) = entry.file_name().to_str() else {
            return false;
        };

        patterns.iter().any(|p| p == file_name)
    }
}

impl Walker for WalkdirWalker {
    fn walk(
        &self,
        source_root: &Path,
        dest_root: &Path,
        ignore_patterns: &[String],
    ) -> Result<Vec<FileMapping>, WalkerError> {
        let mut mappings = Vec::new();

        for entry in WalkDir::new(source_root)
            .into_iter()
            .filter_entry(|e| !Self::should_ignore(e, ignore_patterns))
        {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                continue;
            }

            let Ok(rel) = path.strip_prefix(source_root) else {
                continue;
            };

            let dest = dest_root.join(rel);

            mappings.push(FileMapping {
                source: path.to_path_buf(),
                dest,
            });
        }

        Ok(mappings)
    }
}
