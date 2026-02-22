use std::{
    fs::{self, symlink_metadata},
    path::{Path, PathBuf},
};

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
    /// - the underlying OS call call fails
    /// - the platform does not support symlink creation in the given context
    fn create_symlink(&self, src: &Path, dest: &Path) -> Result<(), FileSystemError>;
}

pub struct ProdFileSystem;

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
            if src.is_dir() {
                std::os::windows::fs::symlink_dir(src, dest)
            } else {
                std::os::windows::fs::symlink_file(src, dest)
            }
            .map_err(|e| FileSystemError::SymLinkError {
                src: src.to_path_buf(),
                dest: dest.to_path_buf(),
                source: e,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashMap, io};

    #[derive(Default)]
    pub struct MockFileSystem {
        files: HashMap<PathBuf, MockEntry>,
    }

    pub enum MockEntry {
        File,
        Dir,
        Symlink(PathBuf),
    }

    impl MockFileSystem {
        pub fn insert_file(&mut self, path: PathBuf) {
            self.files.insert(path, MockEntry::File);
        }
        pub fn insert_dir(&mut self, path: PathBuf) {
            self.files.insert(path, MockEntry::Dir);
        }
        pub fn insert_symlink(&mut self, link: PathBuf, target: PathBuf) {
            self.files.insert(link, MockEntry::Symlink(target));
        }
    }

    impl FileSystemOps for MockFileSystem {
        fn exists(&self, path: &std::path::Path) -> bool {
            self.files.contains_key(path)
        }
        fn is_symlink(&self, path: &std::path::Path) -> bool {
            matches!(self.files.get(path), Some(MockEntry::Symlink(_)))
        }

        fn is_file(&self, path: &Path) -> bool {
            matches!(self.files.get(path), Some(MockEntry::File))
        }

        fn is_dir(&self, path: &Path) -> bool {
            matches!(self.files.get(path), Some(MockEntry::Dir))
        }

        fn read_link(&self, path: &std::path::Path) -> Result<PathBuf, FileSystemError> {
            match self.files.get(path) {
                Some(MockEntry::Symlink(target)) => Ok(target.clone()),
                _ => Err(FileSystemError::LinkError {
                    path: path.to_path_buf(),
                    source: io::Error::other("Failed to read link"),
                }),
            }
        }
        fn create_symlink(&self, _src: &Path, _dest: &Path) -> Result<(), FileSystemError> {
            Ok(())
        }
    }

    #[test]
    fn test_mock_exists() {
        let mut fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");

        fs.insert_file(file.clone());

        assert!(fs.exists(&file));
        assert!(!fs.exists(PathBuf::from("/tmp/missing").as_path()));
    }

    #[test]
    fn test_mock_is_symlink() {
        let mut fs = MockFileSystem::default();
        let link = PathBuf::from("/link");
        let target = PathBuf::from("/target");

        fs.insert_symlink(link.clone(), target);

        assert!(fs.is_symlink(&link));
        assert!(!fs.is_symlink(PathBuf::from("/notalink").as_path()));
    }

    #[test]
    fn test_mock_is_file() {
        let mut fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");
        let dir = PathBuf::from("/tmp/dir/");

        fs.insert_file(file.clone());
        fs.insert_dir(dir.clone());

        assert!(fs.is_file(&file));
        assert!(!fs.is_file(&dir));
    }

    #[test]
    fn test_mock_is_dir() {
        let mut fs = MockFileSystem::default();
        let file = PathBuf::from("/tmp/file");
        let dir = PathBuf::from("/tmp/dir/");

        fs.insert_file(file.clone());
        fs.insert_dir(dir.clone());

        assert!(fs.is_dir(&dir));
        assert!(!fs.is_dir(&file));
    }

    #[test]
    fn test_mock_read_link() {
        let mut fs = MockFileSystem::default();
        let link = PathBuf::from("/dotfiles/.zshrc");
        let target = PathBuf::from("/home/user/.zshrc");

        fs.insert_symlink(link.clone(), target.clone());

        let Ok(resolved) = fs.read_link(&link) else {
            panic!()
        };
        assert_eq!(resolved, target);
    }

    #[test]
    fn test_mock_read_link_failure() {
        let fs = MockFileSystem::default();
        let missing = PathBuf::from("/missing");

        let result = fs.read_link(&missing);
        assert!(result.is_err());
    }
}
