use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Link { src: PathBuf, dest: PathBuf },
    MkDir { path: PathBuf },
    Warn { message: String },
    NoOp,
}
