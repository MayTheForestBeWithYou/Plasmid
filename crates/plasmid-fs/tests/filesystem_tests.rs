use std::{error::Error, fs::File};

use plasmid_fs::filesystem::operation::{FileSystemOps, ProdFileSystem};
use tempfile::TempDir;

#[test]
fn test_exists() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let file_path = dir.path().join("file");
    File::create(file_path.clone())?;

    let fs = ProdFileSystem;

    assert!(fs.exists(&file_path));
    assert!(fs.is_file(&file_path));
    assert!(!fs.is_dir(&file_path));
    assert!(!fs.exists(dir.path().join("missing").as_path()));

    Ok(())
}

#[test]
fn test_symlink_and_read() -> Result<(), Box<dyn Error>> {
    let dir = TempDir::new()?;
    let target = dir.path().join("target");
    let link = dir.path().join("link");
    File::create(&target)?;

    let fs = ProdFileSystem;
    fs.create_symlink(&target, &link)?;

    assert!(fs.exists(&link));

    Ok(())
}
