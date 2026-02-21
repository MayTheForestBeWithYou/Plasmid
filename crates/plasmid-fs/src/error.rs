use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileSystemError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

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
}
