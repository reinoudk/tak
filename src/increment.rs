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
                .validator(validate_increment)
                .help("major|minor|patch|auto"),
        )
}

fn validate_increment(s: String) -> Result<(), String> {
    Increment::try_from(s.as_str()).map(|_| ())
}

pub fn exec(sub_matches: &ArgMatches) {
    let increment = sub_matches.value_of("increment").unwrap();

    println!("Version type: {}", increment);
}
