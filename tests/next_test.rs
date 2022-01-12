use std::env;
use std::process::Command;

use assert_cmd::prelude::*;
use git2::{Repository, Signature, Time};
use tempfile::TempDir;

#[test]
fn test_next_no_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir_in(env::current_dir()?).unwrap();

    tmp_repository(&tmp_dir, None)?;

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next").arg("patch").arg("--no-prefix");
    cmd.assert().success().stdout("1.0.1\n");

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next").arg("minor").arg("--no-prefix");
    cmd.assert().success().stdout("1.1.0\n");

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next").arg("major").arg("--no-prefix");
    cmd.assert().success().stdout("2.0.0\n");

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next").arg("--no-prefix");
    cmd.assert().success().stdout("1.1.0\n");

    // explicitly close tmp_dir so we are notified if it doesn't work
    tmp_dir.close()?;

    Ok(())
}

#[test]
fn test_next_prefix() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir_in(env::current_dir()?).unwrap();

    tmp_repository(&tmp_dir, Some("v"))?;

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next");
    cmd.assert().success().stdout("v1.1.0\n");

    // explicitly close tmp_dir so we are notified if it doesn't work
    tmp_dir.close()?;

    Ok(())
}

#[test]
fn test_write() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir_in(env::current_dir()?).unwrap();

    tmp_repository(&tmp_dir, Some("v"))?;

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("next");
    cmd.arg("-w");
    cmd.assert().success().stdout("v1.1.0\n");

    let repo = match Repository::init(tmp_dir.path()) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let has_new_tag = repo.tag_names(None)?
        .iter()
        .filter_map(|t| t)
        .inspect(|t| println!("Tag: {}", t))
        .find(|s| *s == "v1.1.0")
        .is_some();

    assert!(has_new_tag);

    // explicitly close tmp_dir so we are notified if it doesn't work
    tmp_dir.close()?;

    Ok(())
}

fn tmp_repository(
    tmp_dir: &TempDir,
    prefix: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = match Repository::init(tmp_dir.path()) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let committer = Signature::new("Test", "test@domain.com", &Time::new(0, 0))?;

    let oid = repo.commit(
        Some("HEAD"),
        &committer,
        &committer,
        "Initial commit.",
        &tree,
        &[],
    )?;

    // Only consider SemVer versions
    repo.tag_lightweight(format!("{}1.0.0", prefix.unwrap_or_default()).as_str(), &repo.find_object(oid, None)?, false)?;
    repo.tag_lightweight(format!("{}1.1", prefix.unwrap_or_default()).as_str(), &repo.find_object(oid, None)?, false)?;
    repo.tag_lightweight(format!("{}2", prefix.unwrap_or_default()).as_str(), &repo.find_object(oid, None)?, false)?;

    // Check auto increment
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    repo.commit(
        Some("HEAD"),
        &committer,
        &committer,
        "feat: some feature",
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}
