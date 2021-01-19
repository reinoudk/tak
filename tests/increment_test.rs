use std::env;
use std::process::Command;

use assert_cmd::prelude::*;
use git2::{Repository, Signature, Time};

#[test]
fn test_increment() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir_in(env::current_dir()?).unwrap();

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
    repo.tag_lightweight("1.0.0", &repo.find_object(oid, None)?, false)?;
    repo.tag_lightweight("1.1", &repo.find_object(oid, None)?, false)?;
    repo.tag_lightweight("2", &repo.find_object(oid, None)?, false)?;

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("increment").arg("patch");
    cmd.assert().success().stdout("1.0.1");

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("increment").arg("minor");
    cmd.assert().success().stdout("1.1.0");

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("increment").arg("major");
    cmd.assert().success().stdout("2.0.0");

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

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("increment");
    cmd.assert().success().stdout("1.1.0");

    // explicitly close tmp_dir so we are notified if it doesn't work
    tmp_dir.close()?;

    Ok(())
}
