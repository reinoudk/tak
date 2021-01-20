use std::convert::TryFrom;

use clap::{App, Arg, ArgMatches, SubCommand};

use tak::error::Result;
use tak::git::SemanticRepository;
use tak::increment::Increment;

pub const CMD_NAME: &'static str = "increment";
pub const CMD_ALIASES: &'static [&'static str] = &["inc"];
const INCREMENT_ARG_NAME: &'static str = "increment";

enum IncrementArg {
    PATCH,
    MINOR,
    MAJOR,
    AUTO,
}

impl TryFrom<&str> for IncrementArg {
    type Error = String;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "patch" => Ok(IncrementArg::PATCH),
            "minor" => Ok(IncrementArg::MINOR),
            "major" => Ok(IncrementArg::MAJOR),
            "auto" => Ok(IncrementArg::AUTO),
            _ => Err(String::from(
                "increment should be one of [patch, minor, major, auto]",
            )),
        }
    }
}

pub fn cmd<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CMD_NAME)
        .aliases(CMD_ALIASES)
        .about("create a release")
        .arg(
            Arg::with_name(INCREMENT_ARG_NAME)
                .default_value("auto")
                .validator(validate_increment_arg)
                .help("major|minor|patch|auto"),
        )
}

fn validate_increment_arg(s: String) -> std::result::Result<(), String> {
    IncrementArg::try_from(s.as_str()).and(Ok(()))
}

pub fn exec(sub_matches: &ArgMatches, mut out: impl std::io::Write) -> Result<()> {
    let increment_arg = sub_matches.value_of("increment").unwrap();
    let increment_arg = IncrementArg::try_from(increment_arg).unwrap();

    let repo = SemanticRepository::open().unwrap();

    let new_version = match increment_arg {
        IncrementArg::MAJOR => repo.next_version(Increment::MAJOR),
        IncrementArg::MINOR => repo.next_version(Increment::MINOR),
        IncrementArg::PATCH => repo.next_version(Increment::PATCH),
        IncrementArg::AUTO => repo.automatic_next_version(),
    };

    let new_version = new_version?.to_string();

    Ok(out.write_all(new_version.as_bytes())?)
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
