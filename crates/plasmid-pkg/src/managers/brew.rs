use std::{process::Command, sync::Arc};

use crate::{
    error::PackageManagerError, managers::manager::PackageManager,
    runner::commandrunner::CommandRunner,
};

pub struct BrewManager {
    pub runner: Arc<dyn CommandRunner>,
}

impl BrewManager {
    pub fn new(runner: Arc<dyn CommandRunner>) -> Self {
        Self { runner }
    }
}

impl PackageManager for BrewManager {
    fn name(&self) -> &'static str {
        "brew"
    }

    fn runner(&self) -> &dyn CommandRunner {
        &*self.runner
    }

    fn available(&self) -> Result<bool, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("--version");
        match self.runner().run(&mut cmd) {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    fn install(&self, package: &str) -> Result<String, PackageManagerError> {
        let mut cmd = Command::new(self.name());
        cmd.arg("install").arg(package);
        self.install_cmd(&mut cmd)
    }

    fn is_installed(&self, package: &str) -> Result<bool, PackageManagerError> {
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
            .expect("brew --version")
            .stdout("v1.0.0")
            .status(0)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.available()?;
        assert!(result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_unavailable() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("brew --version")
            .status(1)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.available()?;
        assert!(!result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_install() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("brew install package")
            .stdout("Successfully installed package")
            .status(0)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.install("package");
        assert!(result.is_ok());

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_install_failed() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("brew install package")
            .status(1)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.install("package");
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
            .expect("brew list package")
            .stdout("package v1.0.0")
            .status(0)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.is_installed("package")?;
        assert!(result);

        mock.assert_empty()?;
        Ok(())
    }

    #[test]
    fn test_is_not_installed() -> Result<(), Box<dyn Error>> {
        let mock = MockCommandRunner::builder()
            .expect("brew list package")
            .stdout("package v1.0.0")
            .status(1)
            .finish()
            .build();

        let brew = BrewManager::new(Arc::new(mock.clone()));
        let result = brew.is_installed("package")?;
        assert!(!result);

        mock.assert_empty()?;
        Ok(())
    }
}
