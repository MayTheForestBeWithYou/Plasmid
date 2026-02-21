use std::fs::File;

use plasmid_fs::{
    error::FileSystemError,
    filesystem::{FileSystemOps, ProdFileSystem},
};
use tempfile::TempDir;

#[test]
fn test_exists() -> std::io::Result<()> {
    let dir = TempDir::new()?;
    let file_path = dir.path().join("file");
    let _ = File::create(file_path.clone());

    let fs = ProdFileSystem;

    assert!(fs.exists(&file_path));
    assert!(!fs.exists(dir.path().join("missing").as_path()));

    Ok(())
}

#[test]
fn test_symlink_and_read() -> Result<(), FileSystemError> {
    let dir = TempDir::new()?;
    let target = dir.path().join("target");
    let link = dir.path().join("link");
    let _ = File::create(&target);

    let fs = ProdFileSystem;
    fs.create_symlink(&target, &link)?;

    assert!(fs.is_symlink(&link));

    let resolved = fs.read_link(&link)?;
    assert_eq!(resolved, target);

    Ok(())
}
