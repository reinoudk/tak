use std::convert::TryFrom;

use clap::{App, Arg, ArgMatches, SubCommand};

use tak::error::Result;
use tak::git::SemanticRepository;
use tak::increment::Increment;

pub const CMD_NAME: &'static str = "next";
pub const INCREMENT_ARG_NAME: &'static str = "increment";
pub const NO_PREFIX_ARG_NAME: &'static str = "no-prefix";

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
        .about("show the next version")
        .arg(
            Arg::with_name(INCREMENT_ARG_NAME)
                .default_value("auto")
                .validator(validate_increment_arg)
                .help("major|minor|patch|auto"),
        )
        .arg(
            Arg::with_name(NO_PREFIX_ARG_NAME)
                .long("no-prefix")
                .help("disable 'v' prefix of versions"),

        )
}

fn validate_increment_arg(s: String) -> std::result::Result<(), String> {
    IncrementArg::try_from(s.as_str()).and(Ok(()))
}

pub fn exec(sub_matches: &ArgMatches) -> Result<()> {
    // Unwrapping should always succeed: the increment argument has a default value and is validated
    let increment_arg = sub_matches.value_of(INCREMENT_ARG_NAME).unwrap();
    let increment = IncrementArg::try_from(increment_arg).unwrap();

    let prefix = if sub_matches.is_present(NO_PREFIX_ARG_NAME) { "" } else { "v" };

    let repo = SemanticRepository::open_with_prefix(prefix)?;

    let new_version = match increment {
        IncrementArg::MAJOR => repo.next_version(Increment::MAJOR),
        IncrementArg::MINOR => repo.next_version(Increment::MINOR),
        IncrementArg::PATCH => repo.next_version(Increment::PATCH),
        IncrementArg::AUTO => repo.automatic_next_version(),
    };

    println!("{}{}", prefix, new_version?.to_string());
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
