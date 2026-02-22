use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use crate::filesystem::{error::FileSystemError, operation::FileSystemOps};

pub enum MockEntry {
    File,
    Dir,
    Symlink { target: PathBuf },
}
#[derive(Clone, Default)]
pub struct MockFileSystem {
    entries: Arc<RwLock<HashMap<PathBuf, MockEntry>>>,
}

impl MockFileSystem {
    fn read<F, R>(&self, f: F) -> Result<R, FileSystemError>
    where
        F: FnOnce(&HashMap<PathBuf, MockEntry>) -> R,
    {
        let guard = self
            .entries
            .read()
            .map_err(|_| FileSystemError::LockPoisoned)?;
        Ok(f(&guard))
    }

    fn write<F, R>(&self, f: F) -> Result<R, FileSystemError>
    where
        F: FnOnce(&mut HashMap<PathBuf, MockEntry>) -> R,
    {
        let mut guard = self
            .entries
            .write()
            .map_err(|_| FileSystemError::LockPoisoned)?;
        Ok(f(&mut guard))
    }

    /// # Errors
    ///
    /// This method returns an error if:
    /// - The lock was poisoned.
    pub fn seed_file(&self, path: impl Into<PathBuf>) -> Result<(), FileSystemError> {
        let path = path.into();
        self.ensure_parents(&path)?;
        self.write(|entries| {
            entries.insert(path, MockEntry::File);
        })
    }

    /// # Errors
    ///
    /// This method returns an error if:
    /// - The lock was poisoned.
    pub fn seed_dir(&self, path: impl Into<PathBuf>) -> Result<(), FileSystemError> {
        let path = path.into();
        self.ensure_parents(&path)?;
        self.write(|entries| {
            entries.insert(path, MockEntry::Dir);
        })
    }

    /// # Errors
    ///
    /// This method returns an error if:
    /// - The lock was poisoned.
    fn ensure_parents(&self, path: &Path) -> Result<(), FileSystemError> {
        if let Some(parent) = path.parent() {
            if parent.as_os_str().is_empty() {
                return Ok(());
            }

            self.write(|entries| {
                for ancestor in parent.ancestors() {
                    if ancestor.as_os_str().is_empty() {
                        continue;
                    }
                    entries
                        .entry(ancestor.to_path_buf())
                        .or_insert(MockEntry::Dir);
                }
            })?;
        }
        Ok(())
    }
}

impl FileSystemOps for MockFileSystem {
    fn exists(&self, path: &Path) -> bool {
        self.read(|entries| entries.contains_key(path))
            .unwrap_or(false)
    }
    fn is_symlink(&self, path: &std::path::Path) -> bool {
        self.read(|entries| matches!(entries.get(path), Some(MockEntry::Symlink { .. })))
            .unwrap_or(false)
    }

    fn is_file(&self, path: &Path) -> bool {
        self.read(|entries| matches!(entries.get(path), Some(MockEntry::File)))
            .unwrap_or(false)
    }

    fn is_dir(&self, path: &Path) -> bool {
        self.read(|entries| matches!(entries.get(path), Some(MockEntry::Dir)))
            .unwrap_or(false)
    }

    fn read_link(&self, path: &std::path::Path) -> Result<PathBuf, FileSystemError> {
        self.read(|entries| match entries.get(path) {
            Some(MockEntry::Symlink { target }) => Ok(target.clone()),
            Some(_) => Err(FileSystemError::InvalidOperation {
                path: path.to_path_buf(),
                details: "Not a symlink".into(),
            }),
            None => Err(FileSystemError::MockError {
                msg: format!("Path not found: {}", path.display()),
            }),
        })?
    }

    fn create_symlink(&self, src: &Path, dest: &Path) -> Result<(), FileSystemError> {
        if !self.exists(src) {
            return Err(FileSystemError::MockError {
                msg: format!("Source not found: {}", src.display()),
            });
        }

        self.ensure_parents(dest)?;

        self.write(|entries| {
            entries.insert(
                dest.to_path_buf(),
                MockEntry::Symlink {
                    target: src.to_path_buf(),
                },
            );
        })
    }

    fn create_dir(&self, path: &Path) -> Result<(), FileSystemError> {
        if self.exists(path) {
            return Err(FileSystemError::MockError {
                msg: format!("Path already exists: {}", path.display()),
            });
        }

        self.write(|entries| {
            entries.insert(path.to_path_buf(), MockEntry::Dir);
        })
    }

    fn walk(&self, root: &Path, ignore: &[String]) -> Result<Vec<PathBuf>, FileSystemError> {
        self.read(|entries| {
            let mut results = Vec::new();

            for path in entries.keys() {
                if !path.starts_with(root) {
                    continue;
                }

                let path_str = path.to_string_lossy();
                let is_ignored = ignore.iter().any(|p| path_str.contains(p));

                if !is_ignored {
                    results.push(path.clone());
                }
            }

            results.sort();
            results
        })
    }
}
