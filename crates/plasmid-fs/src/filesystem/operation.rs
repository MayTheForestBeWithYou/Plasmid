use std::{
    fs::{self, symlink_metadata},
    path::{Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};

use crate::filesystem::error::FileSystemError;

/// Abstraction layer for filesystem operations.
pub trait FileSystemOps {
    /// Checks whether a path exists in the filesystem.
    fn exists(&self, path: &Path) -> bool;

    /// Determines whether a path is a symbolic link.
    fn is_symlink(&self, path: &Path) -> bool;

    /// Determines whether a path is a file.
    fn is_file(&self, path: &Path) -> bool;

    /// Determines whether a path is a directory;
    fn is_dir(&self, path: &Path) -> bool;

    /// Reads the target of a symbolic link.
    ///
    /// # Errors
    /// This method returns an error if:
    /// - the path does not exist
    /// - the underyling filesystem call fails
    fn read_link(&self, path: &Path) -> Result<PathBuf, FileSystemError>;

    /// Creates a symbolic link at `dest` pointing to `src`.
    ///
    /// # Errors
    /// This method returns an error if:
    /// - the destination path cannot be created
    /// - the underlying OS call fails
    /// - the platform does not support symlink creation in the given context
    fn create_symlink(&self, src: &Path, dest: &Path) -> Result<(), FileSystemError>;

    /// Creates a directory.
    ///
    /// # Errors
    /// This method returns an error if:
    /// - the underyling OS call fails
    fn create_dir(&self, path: &Path) -> Result<(), FileSystemError>;

    /// Walks over directory recursively to discover all files that do not match ignore.
    ///
    /// # Errors
    /// This method returns an error if:
    /// - Walkdir crate fails.
    fn walk(&self, root: &Path, ignore: &[String]) -> Result<Vec<PathBuf>, FileSystemError>;
}

pub struct ProdFileSystem;

impl ProdFileSystem {
    fn should_ignore(entry: &DirEntry, patterns: &[String]) -> bool {
        let Some(file_name) = entry.file_name().to_str() else {
            return false;
        };

        patterns.iter().any(|p| p == file_name)
    }
}

impl FileSystemOps for ProdFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn is_symlink(&self, path: &Path) -> bool {
        symlink_metadata(path).is_ok_and(|meta| meta.file_type().is_symlink())
    }

    fn is_file(&self, path: &Path) -> bool {
        path.is_file()
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn read_link(&self, path: &Path) -> Result<PathBuf, FileSystemError> {
        fs::read_link(path).map_err(|e| FileSystemError::LinkError {
            path: path.to_path_buf(),
            source: e,
        })
    }
    fn create_symlink(&self, src: &Path, dest: &Path) -> Result<(), FileSystemError> {
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(src, dest).map_err(|e| FileSystemError::SymLinkError {
                src: src.to_path_buf(),
                dest: dest.to_path_buf(),
                source: e,
            })
        }
        #[cfg(windows)]
        {
            let symlink_result = if src.is_dir() {
                std::os::windows::fs::symlink_dir(src, dest)
            } else {
                std::os::windows::fs::symlink_file(src, dest)
            };

            match symlink_result {
                Ok(()) => Ok(()),
                Err(e) => {
                    let Some(raw) = e.raw_os_error() else {
                        return Err(e).map_err(|err| FileSystemError::SymLinkError {
                            src: src.to_path_buf(),
                            dest: dest.to_path_buf(),
                            source: err,
                        });
                    };

                    let privilige_issue = raw == 1314 || raw == 87;

                    if privilige_issue && src.is_file() {
                        return std::fs::hard_link(src, dest).map_err(|err| {
                            FileSystemError::HardLinkError {
                                src: src.to_path_buf(),
                                dest: dest.to_path_buf(),
                                source: err,
                            }
                        });
                    }

                    Err(e)?
                }
            }
        }
    }

    fn create_dir(&self, path: &Path) -> Result<(), FileSystemError> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    fn walk(&self, root: &Path, ignore: &[String]) -> Result<Vec<PathBuf>, FileSystemError> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| !Self::should_ignore(e, ignore))
        {
            let entry = entry?;
            let path = entry.path();

            if self.is_dir(path) {
                continue;
            }

            files.push(path.to_path_buf());
        }
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::filesystem::mock::MockFileSystem;

    #[test]
    fn test_mock_exists() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");

        fs.seed_file(file.clone())?;

        assert!(fs.exists(&file));
        assert!(!fs.exists(PathBuf::from("/tmp/missing").as_path()));
        Ok(())
    }

    #[test]
    fn test_mock_is_symlink() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let link = PathBuf::from("/link");
        let target = PathBuf::from("/target");

        fs.seed_file(target.clone())?;
        fs.create_symlink(&target, &link)?;

        assert!(fs.is_symlink(&link));
        assert!(!fs.is_symlink(PathBuf::from("/notalink").as_path()));
        Ok(())
    }

    #[test]
    fn test_mock_is_file() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");
        let dir = PathBuf::from("/tmp/dir/");

        fs.seed_file(file.clone())?;
        fs.seed_dir(dir.clone())?;

        assert!(fs.is_file(&file));
        assert!(!fs.is_file(&dir));
        Ok(())
    }

    #[test]
    fn test_mock_is_dir() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");
        let dir = PathBuf::from("/tmp/dir/");

        fs.seed_file(file.clone())?;
        fs.seed_dir(dir.clone())?;

        assert!(fs.is_dir(&dir));
        assert!(!fs.is_dir(&file));
        Ok(())
    }

    #[test]
    fn test_mock_read_link() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let link = PathBuf::from("/dotfiles/.zshrc");
        let target = PathBuf::from("/home/user/.zshrc");

        fs.seed_file(target.clone())?;
        fs.create_symlink(&target, &link)?;

        let resolved = fs.read_link(&link)?;
        assert_eq!(resolved, target);

        Ok(())
    }

    #[test]
    fn test_mock_read_link_failure() {
        let fs = MockFileSystem::default();
        let missing = PathBuf::from("/missing");

        let result = fs.read_link(&missing);
        assert!(result.is_err());
    }

    #[test]
    fn test_mock_create_symlink() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let src = PathBuf::from("src.txt");
        let dest = PathBuf::from("dest.txt");
        fs.seed_file(&src)?;

        let result = fs.create_symlink(&src, &dest);
        assert!(result.is_ok());
        assert!(fs.is_symlink(&dest));
        Ok(())
    }

    #[test]
    fn test_mock_create_dir() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let path = PathBuf::from("/dir");

        fs.create_dir(&path)?;

        assert!(fs.exists(&path));
        Ok(())
    }

    #[test]
    fn test_mock_walk_dir() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let root = PathBuf::from("/root");
        let file_a = PathBuf::from("/root/a.txt");
        fs.seed_dir(&root)?;
        fs.seed_file(&file_a)?;

        let result = fs.walk(&root, &[])?;
        assert!(result.contains(&file_a));
        Ok(())
    }

    #[test]
    fn test_mock_walk_dir_ignore() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let root = PathBuf::from("/root");
        let file_a = PathBuf::from("/root/a.txt");
        let file_b = PathBuf::from("/root/b.txt");
        fs.seed_dir(&root)?;
        fs.seed_file(&file_a)?;
        fs.seed_file(&file_b)?;

        let result = fs.walk(&root, &["b.txt".into()])?;
        assert!(result.contains(&file_a));
        assert!(!result.contains(&file_b));
        Ok(())
    }
}
