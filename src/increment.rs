use clap::{App, Arg, ArgMatches, SubCommand};
use std::convert::TryFrom;

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

pub fn exec(sub_matches: &ArgMatches) {
    let increment = sub_matches.value_of("increment").unwrap();

    println!("Version type: {}", increment);
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
        }
    }

    macro_rules! test_validation_fail {
        ( $name:ident, $s:expr) => {
            #[test]
            fn $name() {
                let arg = String::from($s);
                let result = validate_increment_arg(arg);
                assert!(result.is_err());
            }
        }
    }

    test_validation_ok!(major_works, "major");
    test_validation_ok!(minor_works, "minor");
    test_validation_ok!(patch_works, "patch");
    test_validation_ok!(auto_works, "auto");

    test_validation_fail!(bogus_does_not_work, "bogus");
}
