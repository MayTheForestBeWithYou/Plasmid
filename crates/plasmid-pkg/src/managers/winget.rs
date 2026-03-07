use std::{process::Command, sync::Arc};

use crate::{
    error::PackageManagerError, managers::manager::PackageManager,
    runner::commandrunner::CommandRunner,
};

pub struct WingetManager {
    pub runner: Arc<dyn CommandRunner>,
}

impl WingetManager {
    pub fn new(runner: Arc<dyn CommandRunner>) -> Self {
        Self { runner }
    }
}

impl PackageManager for WingetManager {
    fn name(&self) -> &'static str {
        "winget"
    }

    fn runner(&self) -> &dyn CommandRunner {
        &*self.runner
    }

    fn is_available(&self) -> Result<bool, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("--version");
        match self.runner().run(&mut cmd) {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    fn package_available(&self, package: &str) -> Result<bool, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("show").arg(package);
        match self.runner().run(&mut cmd) {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    fn install_package(&self, package: &str) -> Result<String, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("install").arg(package);
        self.install_cmd(&mut cmd)
    }

    fn is_package_installed(&self, package: &str) -> Result<bool, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("list").arg(package);

        match self.runner().run(&mut cmd) {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::mock::MockCommandRunner;
    use std::error::Error;

    #[test]
    fn test_available() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget --version")
            .stdout("v1.0.0")
            .status(0)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.is_available()?;
        assert!(result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_unavailable() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget --version")
            .status(1)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.is_available()?;
        assert!(!result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_package_available() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget show package")
            .status(0)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.package_available("package")?;
        assert!(result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_package_unavailable() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget show package")
            .status(1)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.package_available("package")?;
        assert!(!result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_install() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget install package")
            .stdout("Successfully installed package")
            .status(0)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.install_package("package");
        assert!(result.is_ok());

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_install_failed() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget install package")
            .status(1)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.install_package("package");
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(PackageManagerError::InstallFailed(_, _))
        ));

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_is_installed() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget list package")
            .stdout("package v1.0.0")
            .status(0)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.is_package_installed("package")?;
        assert!(result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_is_not_installed() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("winget list package")
            .stdout("package v1.0.0")
            .status(1)
            .finish()
            .build();

        let winget = WingetManager::new(Arc::new(mock.clone()));
        let result = winget.is_package_installed("package")?;
        assert!(!result);

        mock.assert_empty()?;
        Ok(())
    }
}
