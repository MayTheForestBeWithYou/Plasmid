use plasmid_fs::filesystem::error::FileSystemError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("Filesystem error: {0}")]
    FileSystemError(#[from] FileSystemError),
}
