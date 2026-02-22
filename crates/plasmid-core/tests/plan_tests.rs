use std::{error::Error, fs::File, path::PathBuf};

use plasmid_core::{MockConfigReader, loader::load_config, planner::core::Planner};
use plasmid_fs::filesystem::operation::{FileSystemOps, ProdFileSystem};
use tempfile::TempDir;

#[test]
fn test_planner_resolves_paths() -> Result<(), Box<dyn Error>> {
    let fs = ProdFileSystem;
    let repo_root_binding = TempDir::new()?;
    let home_dir_binding = TempDir::new()?;
    let repo_root = repo_root_binding.path();
    let home_dir = home_dir_binding.path();

    File::create(repo_root.join(".bashrc"))?;
    fs.create_dir(&repo_root.join(".config"))?;
    File::create(repo_root.join(".config").join("plasmid.config"))?;
    let reader = MockConfigReader::default();
    let config = load_config(&PathBuf::from("nonexistent"), &reader)?;

    let planner = Planner::new(&fs, repo_root.to_path_buf(), home_dir.to_path_buf());
    let plan = planner.build(&config)?;

    assert_eq!(plan.operations.len(), 3);

    plan.print();
    plan.execute(&fs)?;

    assert!(fs.exists(&home_dir.join(".bashrc")));
    assert!(fs.is_file(&home_dir.join(".bashrc")));
    assert!(fs.exists(&home_dir.join(".config").join("plasmid.config")));
    assert!(fs.is_dir(&home_dir.join(".config")));
    assert!(fs.is_file(&home_dir.join(".config").join("plasmid.config")));

    Ok(())
}
