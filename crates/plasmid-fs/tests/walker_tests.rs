use std::{
    error::Error,
    fs::{File, create_dir},
};

use plasmid_fs::filesystem::operation::{FileSystemOps, ProdFileSystem};
use tempfile::TempDir;

#[test]
fn test_basic_mapping() -> Result<(), Box<dyn Error>> {
    let fs = ProdFileSystem;
    let src = TempDir::new()?;
    let file_a = src.path().join("a.txt");
    let file_b = src.path().join("nested/b.txt");

    File::create(&file_a)?;
    create_dir(src.path().join("nested"))?;
    File::create(&file_b)?;

    let mappings = fs.walk(src.path(), &[])?;

    assert_eq!(mappings.len(), 2);
    assert!(mappings.contains(&file_a));
    assert!(mappings.contains(&file_b));
    Ok(())
}

#[test]
fn test_ignore_patterns() -> Result<(), Box<dyn Error>> {
    let fs = ProdFileSystem;
    let src = TempDir::new()?;

    create_dir(src.path().join(".git"))?;
    File::create(src.path().join(".git/config"))?;

    let mappings = fs.walk(src.path(), &[".git".into()])?;

    assert!(mappings.is_empty());
    Ok(())
}

#[test]
fn test_ignores_directories() -> Result<(), Box<dyn Error>> {
    let fs = ProdFileSystem;
    let src = TempDir::new()?;

    create_dir(src.path().join("dir"))?;

    let mappings = fs.walk(src.path(), &[])?;

    assert!(mappings.is_empty());
    Ok(())
}
