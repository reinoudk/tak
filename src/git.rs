use core::cmp;

use git2::{Commit, Repository};
use semver::Version;

use crate::conventional_commits;
use crate::error::{Error, Result};
use crate::increment::Increment;

pub struct SemanticRepository {
    repository: Repository,
}

impl SemanticRepository {
    pub fn open() -> Result<Self> {
        let dir = std::env::current_dir()?;
        let repository = Repository::open(dir)?;

        Ok(Self { repository })
    }

    pub fn highest_version(&self) -> Result<Version> {
        let initial_version: Option<Version> = None;

        self.repository
            .tag_names(None)?
            .iter()
            .filter_map(|s| s)
            .filter_map(|s| Version::parse(s).ok())
            .fold(
                initial_version,
                |highest: Option<Version>, version: Version| match (highest, version) {
                    (Some(highest), version) => Some(cmp::max(highest, version)),
                    (None, version) => Some(version),
                },
            )
            .ok_or(Error::NoTagFound)
    }

    fn commits_since(&self, version: Version) -> Result<Vec<Commit>> {
        let mut walk = self.repository.revwalk()?;
        walk.push_range(&format!("{}..HEAD", version))?;
        let commits: Vec<Commit> = walk
            .filter_map(std::result::Result::ok)
            .filter_map(|oid| {
                return self.repository.find_commit(oid).ok();
            })
            .collect();

        Ok(commits)
    }

    pub fn next_version(&self, increment: Increment) -> Result<Version> {
        let current_version = self.highest_version()?;

        let mut new_version = current_version.clone();

        match increment {
            Increment::MAJOR => new_version.increment_major(),
            Increment::MINOR => new_version.increment_minor(),
            Increment::PATCH => new_version.increment_patch(),
            Increment::NONE => (),
        };

        Ok(new_version)
    }

    pub fn automatic_next_version(&self) -> Result<Version> {
        let current_version = self.highest_version()?;

        let commits = self.commits_since(current_version)?;
        let messages = commits.iter().filter_map(Commit::message);
        let increment = conventional_commits::max_semantic_increment(messages);

        self.next_version(increment)
    }
}
