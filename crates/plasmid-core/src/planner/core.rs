use std::{collections::HashSet, io, path::PathBuf};

use plasmid_fs::filesystem::{error::FileSystemError, operation::FileSystemOps};

use crate::{
    config::schema::PlasmidConfig,
    planner::{error::PlannerError, operation::Operation, plan::Plan},
};

pub struct Planner<'a> {
    fs: &'a dyn FileSystemOps,
    repo_root: PathBuf,
    home_dir: PathBuf,
}

impl<'a> Planner<'a> {
    pub fn new(fs: &'a dyn FileSystemOps, repo_root: PathBuf, home_dir: PathBuf) -> Self {
        Self {
            fs,
            repo_root,
            home_dir,
        }
    }

    /// Builds the plan by walking src directory and compiling operations
    ///
    /// # Errors
    /// This method returns an error if:
    /// - Multiple sources for same destinations
    /// - Fails to calculate relative path
    pub fn build(&self, config: &PlasmidConfig) -> Result<Plan, PlannerError> {
        let mut plan = Plan::new();

        let mut planned_destinations: HashSet<PathBuf> = HashSet::new();

        let paths_to_link = self.fs.walk(&self.repo_root, &config.ignore)?;
        for item in paths_to_link {
            let relative = item.strip_prefix(&self.repo_root).map_err(|_| {
                FileSystemError::Io(io::Error::other("Failed to calculate relative path"))
            })?;
            let item_dst = &self.home_dir.join(relative);

            if planned_destinations.contains(item_dst) {
                return Err(FileSystemError::Io(io::Error::other(format!(
                    "Multiple sources map to {}",
                    item_dst.display()
                ))))?;
            }

            planned_destinations.insert(item_dst.clone());

            if let Some(parent) = item_dst.parent()
                && !self.fs.exists(parent)
                && let Some(parent_src) = item.parent()
                && self.fs.is_dir(parent_src)
            {
                plan.add(Operation::MkDir {
                    path: parent.to_path_buf(),
                });
            }

            if !self.fs.exists(item_dst) && self.fs.is_file(&item) {
                plan.add(Operation::Link {
                    src: item,
                    dest: item_dst.clone(),
                });
            }
        }
        Ok(plan)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, path::PathBuf};

    use plasmid_fs::filesystem::{mock::MockFileSystem, operation::FileSystemOps};

    use crate::{config::mock::MockConfigReader, loader::load_config, planner::core::Planner};

    #[test]
    fn test_planner_resolves_mock_paths() -> Result<(), Box<dyn Error>> {
        let fs = MockFileSystem::default();
        let repo_root = PathBuf::from("/mock/app_dir");
        let home_dir = PathBuf::from("/mock/home_dir");

        fs.seed_dir(&repo_root)?;
        fs.seed_dir(&home_dir)?;
        fs.seed_file(repo_root.join(".bashrc"))?;
        fs.seed_dir(repo_root.join(".config"))?;
        fs.seed_file(repo_root.join(".config").join("plasmid.config"))?;
        let reader = MockConfigReader::default();
        let config = load_config(&PathBuf::from("nonexistent"), &reader)?;

        let planner = Planner::new(&fs, repo_root, home_dir.clone());
        let plan = planner.build(&config)?;

        assert_eq!(plan.operations.len(), 3);

        plan.print();
        plan.execute(&fs)?;

        assert!(fs.exists(&home_dir.join(".bashrc")));
        assert!(fs.is_symlink(&home_dir.join(".bashrc")));
        assert!(fs.exists(&home_dir.join(".config").join("plasmid.config")));
        assert!(fs.is_dir(&home_dir.join(".config")));
        assert!(fs.is_symlink(&home_dir.join(".config").join("plasmid.config")));

        Ok(())
    }
}
