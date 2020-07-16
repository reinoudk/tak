use std::cmp;
use std::convert::TryFrom;
use std::io::Error;

use clap::{App, Arg, ArgMatches, SubCommand};
use git2::Repository;
use semver::Version;

pub const CMD_NAME: &'static str = "increment";
const INCREASE_ARG_NAME: &'static str = "increment";

enum Increment {
    PATCH,
    MINOR,
    MAJOR,
    AUTO,
}

impl TryFrom<&str> for Increment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "patch" => Ok(Increment::PATCH),
            "minor" => Ok(Increment::MINOR),
            "major" => Ok(Increment::MAJOR),
            "auto" => Ok(Increment::AUTO),
            _ => Err(String::from(
                "increment should be one of [patch, minor, major, auto]",
            )),
        }
    }
}

pub fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_NAME)
        .about("create a release")
        .arg(
            Arg::with_name(INCREASE_ARG_NAME)
                .default_value("auto")
                .validator(validate_increment_arg)
                .help("major|minor|patch|auto"),
        )
}

fn validate_increment_arg(s: String) -> Result<(), String> {
    Increment::try_from(s.as_str()).map(|_| ())
}

pub fn exec(sub_matches: &ArgMatches, mut out: impl std::io::Write) -> Result<(), Error> {
    let increment = sub_matches.value_of("increment").unwrap();
    let increment = Increment::try_from(increment).unwrap();

    let current_dir = std::env::current_dir()?;
    let repo = Repository::open(current_dir).unwrap();

    let current_version = highest_tag(&repo).unwrap();
    let current_version = current_version.unwrap();

    let mut new_version = current_version.clone();

    match increment {
        Increment::MAJOR => new_version.increment_major(),
        Increment::MINOR => new_version.increment_minor(),
        Increment::PATCH => new_version.increment_patch(),
        _ => (),
    };

    let new_version = new_version.to_string();

    out.write_all(new_version.as_bytes())
}

fn highest_tag(repo: &Repository) -> Result<Option<Version>, git2::Error> {
    // let head = repo.head()?;

    let initial_version: Option<Version> = None;

    let version = repo
        .tag_names(None)?
        .iter()
        .filter_map(|s| s)
        .map(|s| s)
        .filter_map(|s| Version::parse(s).ok())
        .fold(
            initial_version,
            |state: Option<Version>, version: Version| match (state, version) {
                (Some(state), version) => Some(cmp::max(state, version)),
                (None, version) => Some(version),
            },
        );

    Ok(version)
}

fn _info_tag_or_commit(repo: Repository, version: Option<Version>) -> Result<(), git2::Error>{
    if let Some(version) = version {
        let obj = repo.revparse_single(&version.to_string())?;

        if let Some(tag) = obj.as_tag() {
            println!("Test Tag: {} ({})", tag.name().unwrap(), version)
        } else if let Some(commit) = obj.as_commit() {
            println!("Test Commit: {} ({})", commit.id().to_string(), version)
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_validation_ok {
        ( $name:ident, $s:expr) => {
            #[test]
            fn $name() {
                let arg = String::from($s);
                let result = validate_increment_arg(arg);
                assert!(result.is_ok());
            }
        };
    }

    test_validation_ok!(major_works, "major");
    test_validation_ok!(minor_works, "minor");
    test_validation_ok!(patch_works, "patch");
    test_validation_ok!(auto_works, "auto");

    macro_rules! test_validation_fail {
        ( $name:ident, $s:expr) => {
            #[test]
            fn $name() {
                let arg = String::from($s);
                let result = validate_increment_arg(arg);
                assert!(result.is_err());
            }
        };
    }

    test_validation_fail!(bogus_does_not_work, "bogus");
}
