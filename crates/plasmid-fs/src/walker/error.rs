use thiserror::Error;

#[derive(Debug, Error)]
pub enum WalkerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Walkdir error: {0}")]
    Walkdir(#[from] walkdir::Error),
}
