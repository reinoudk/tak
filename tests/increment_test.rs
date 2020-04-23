use assert_cmd::prelude::*;
use git2::{Repository, Signature, Time};
use std::env;
use std::process::Command;
use std::time::Duration;

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

    repo.tag_lightweight("1.0", &repo.find_object(oid, None)?, false);
    repo.tag_lightweight("1.0.0", &repo.find_object(oid, None)?, false);
    repo.tag_lightweight("2", &repo.find_object(oid, None)?, false);

    let mut cmd = Command::cargo_bin("tak")?;
    cmd.current_dir(tmp_dir.path());
    cmd.arg("increment").arg("patch");
    cmd.assert().success().stdout("1.0.1");

    // explicitly close tmp_dir so we are notified if it doesn't work
    tmp_dir.close()?;

    Ok(())
}
