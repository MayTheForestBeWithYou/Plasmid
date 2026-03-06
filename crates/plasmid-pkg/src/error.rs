use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackageManagerError {
    #[error("Package manager '{0}' is not available on this system")]
    NotAvailable(String),

    #[error("Failed to install package '{0}': {1}")]
    InstallFailed(String, String),

    #[error("Failed to check installation status for '{0}': {1}")]
    CheckFailed(String, String),

    #[error("Unsupported OS or architecture")]
    UnsupportedSystem,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("UTF-8 Conversion Error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("mutext lock failed inside mock command runner: {0}")]
    LockError(String),
}
