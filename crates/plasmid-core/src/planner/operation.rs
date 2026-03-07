use colored::Colorize;
use plasmid_fs::filesystem::operation::FileSystemOps;
use std::path::{MAIN_SEPARATOR, Path, PathBuf};

use crate::planner::error::PlannerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Link { src: PathBuf, dest: PathBuf },
    MkDir { path: PathBuf },
}

impl Operation {
    pub fn pretty_print(&self) {
        match self {
            Self::MkDir { path } => {
                println!(
                    "  {} {}",
                    "+ DIR ".green().bold(),
                    Self::normalize_path_to_os(path)
                );
            }
            Self::Link { src, dest } => {
                println!(
                    "  {} {} -> {}",
                    "+ LINK".cyan().bold(),
                    Self::normalize_path_to_os(src),
                    Self::normalize_path_to_os(dest)
                );
            }
        }
    }

    /// Execute the stored operation
    ///
    /// # Errors
    /// This method returns an error if:
    /// - the underyling fs call fails
    pub fn execute(&self, fs: &dyn FileSystemOps) -> Result<(), PlannerError> {
        match self {
            Self::MkDir { path } => fs.create_dir(path)?,
            Self::Link { src, dest } => {
                if fs.exists(dest) {
                    println!("Target exists");
                    return Ok(());
                }
                fs.create_symlink(src, dest)?;
            }
        }
        Ok(())
    }

    fn normalize_path_to_os(path: &Path) -> String {
        let mut out = String::new();
        for (i, comp) in path.components().enumerate() {
            if i > 1 {
                out.push(MAIN_SEPARATOR);
            }
            out.push_str(&comp.as_os_str().to_string_lossy());
        }

        out
    }
}
