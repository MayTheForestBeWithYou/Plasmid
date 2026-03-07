use std::process::Command;

use crate::{error::PackageManagerError, runner::commandrunner::CommandRunner};

pub trait PackageManager {
    /// The unqiue name
    fn name(&self) -> &'static str;

    /// Command runner injection
    fn runner(&self) -> &dyn CommandRunner;

    /// Checks if the package manager binary exists in PATH
    ///
    /// # Errors
    /// This method returns an error if:
    /// - The proces fails exucting the command
    fn available(&self) -> Result<bool, PackageManagerError>;

    /// Installs the package
    ///
    /// # Errors
    /// This methods returns an error if:
    /// - The process fails executing the command
    fn install(&self, package: &str) -> Result<String, PackageManagerError>;

    /// Checks if a specific package is currently installed
    ///
    /// # Errors
    /// This method returns an error if:
    /// - The process fails executing the command
    fn is_installed(&self, package: &str) -> Result<bool, PackageManagerError>;

    /// Helper to run shell commands cleanly
    ///
    /// # Errors
    /// This method returns an error if:
    /// - The process fails executing the command
    fn install_cmd(&self, cmd: &mut Command) -> Result<String, PackageManagerError> {
        let output = self.runner().run(cmd)?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?)
        } else {
            let stderr = String::from_utf8(output.stderr)?;
            Err(PackageManagerError::InstallFailed(
                self.name().to_string(),
                stderr,
            ))
        }
    }
}
