use std::fs::{File, create_dir};

use plasmid_fs::walker::{
    error::WalkerError,
    walkdir::{FileMapping, WalkdirWalker, Walker},
};
use tempfile::TempDir;

#[test]
fn test_basic_mapping() -> Result<(), WalkerError> {
    let src = TempDir::new()?;
    let dst = TempDir::new()?;

    let file_a = src.path().join("a.txt");
    let file_b = src.path().join("nested/b.txt");

    let _ = File::create(&file_a);
    let _ = create_dir(src.path().join("nested"));
    let _ = File::create(&file_b);

    let walker = WalkdirWalker;

    let mappings = walker.walk(src.path(), dst.path(), &[])?;

    assert_eq!(mappings.len(), 2);

    assert!(mappings.contains(&FileMapping {
        source: file_a,
        dest: dst.path().join("a.txt"),
    }));

    assert!(mappings.contains(&FileMapping {
        source: file_b,
        dest: dst.path().join("nested/b.txt")
    }));

    Ok(())
}

#[test]
fn test_ignore_patterns() -> Result<(), WalkerError> {
    let src = TempDir::new()?;
    let dst = TempDir::new()?;

    let _ = create_dir(src.path().join(".git"));
    let _ = File::create(src.path().join(".git/config"));

    let walker = WalkdirWalker;

    let mappings = walker.walk(src.path(), dst.path(), &[".git".into()])?;

    assert!(mappings.is_empty());

    Ok(())
}

#[test]
fn test_ignores_directories() -> Result<(), WalkerError> {
    let src = TempDir::new()?;
    let dst = TempDir::new()?;

    let _ = create_dir(src.path().join("dir"));

    let walker = WalkdirWalker;

    let mappings = walker.walk(src.path(), dst.path(), &[])?;

    assert!(mappings.is_empty());

    Ok(())
}
