use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walkdir error: {0}")]
    WalkError(#[from] walkdir::Error),

    #[error("Failed to read link '{path}': {source}")]
    LinkError {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to create symlink between {src} and {dest}: {source}")]
    SymLinkError {
        src: PathBuf,
        dest: PathBuf,
        source: std::io::Error,
    },

    #[error("Failed to create hardlink between {src} and {dest}: {source}")]
    HardLinkError {
        src: PathBuf,
        dest: PathBuf,
        source: std::io::Error,
    },

    #[error("Invalid operation at {path}: {details}")]
    InvalidOperation { path: PathBuf, details: String },

    #[cfg(any(test, feature = "mock"))]
    #[error("Mock error: {msg}")]
    MockError { msg: String },

    #[cfg(any(test, feature = "mock"))]
    #[error("Internal Error: Mock filesystem lock is poisoned")]
    LockPoisoned,
}
