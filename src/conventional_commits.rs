use std::cmp;
use std::str::FromStr;

use regex::Regex;

use crate::increment::Increment;

pub struct ConventionalCommit {
    change_type: String,
    scope: Option<String>,
    is_breaking: bool,
    short_description: String
}

impl FromStr for ConventionalCommit {
    // TODO: Conventional Commit Parse Error
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
              ^(?P<change_type>feat|fix|BREAKING\ CHANGE) # Type
              (?:\((?P<scope>.+)\))? # Scope (optional, without surrounding '()')
              (?P<breaking>!)? # Breaking change indicator (optional)
              (?::\ ) # Mandatory colon and space
              (?P<short_description>.+) # Short commit description
              ( # Ensure that optional commit body does not gobble up everything
                (?:(?s:.*))? # Commit body (optional)
                (?P<breaking_footer>(:?BREAKING-CHANGE|BREAKING\ CHANGE):\ ) # Breaking change in footer (optional, without surrounding '()')
              )?"
            )
            .unwrap();
        }

        let commit : Result<Self, Self::Err>;

        if let Some(caps) = RE.captures(s) {
            let change_type = caps.name("change_type").unwrap().as_str().to_string();
            let scope = caps.name("scope").map(|s| s.as_str().to_string());
            let breaking = caps.name("breaking").is_some() || caps.name("breaking_footer").is_some();
            let short_description = caps.name("short_description").unwrap().as_str().to_string();

            commit = Ok(ConventionalCommit {
                change_type,
                scope,
                is_breaking: breaking,
                short_description,
            });
        } else {
            commit = Err(String::from("could not parse string into conventional commit"));
        }

        commit
    }
}

pub fn max_semantic_increment<'a, I: Iterator<Item = &'a str>>(messages: I) -> Increment {
    let increment = messages.fold(Increment::NONE, |acc, message| {
        let increment = semantic_increment(message);

        // return the biggest increment type
        return cmp::max(acc, increment);
    });
    increment
}

fn semantic_increment(message: &str) -> Increment {
    let mut increment = Increment::NONE;

    if let Ok(commit) = message.parse::<ConventionalCommit>() {
        if commit.is_breaking {
            // Increase major on breaking change
            increment = Increment::MAJOR;
        } else {
            // Use the value of type to determine increment
            increment = match commit.change_type.as_str() {
                "fix" => Increment::PATCH,
                "feat" => Increment::MINOR,
                _ => Increment::NONE,
            }
        }
    }
    increment
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_semantic_increment {
        ( $name:ident, $message:expr, $want:expr) => {
            #[test]
            fn $name() {
                let message = $message;
                let increment = semantic_increment(message);
                assert_eq!(
                    increment, $want,
                    "The increment for '{}' should be '{:?}'",
                    message, $want
                )
            }
        };
    }

    test_semantic_increment!(fix_results_in_patch, "fix: description\n", Increment::PATCH);
    test_semantic_increment!(
        feat_results_in_minor,
        "feat: description\n",
        Increment::MINOR
    );
    test_semantic_increment!(
        exclamation_mark_results_in_major,
        "fix!: description\n",
        Increment::MAJOR
    );
    test_semantic_increment!(
        breaking_change_with_hyphen_in_footer_results_in_major,
        "fix: description\n\nBREAKING-CHANGE: ",
        Increment::MAJOR
    );
    test_semantic_increment!(
        breaking_change_without_hyphen_in_footer_results_in_major,
        "fix: description\n\nBREAKING CHANGE: ",
        Increment::MAJOR
    );
    test_semantic_increment!(
        breaking_change_in_footer_without_newline_results_in_major,
        "fix: description\nBREAKING-CHANGE: ",
        Increment::MAJOR
    );
    test_semantic_increment!(
        breaking_change_in_footer_after_body_results_in_major,
        "fix: description\nsome body\n\nonce told me\n\n\n\nBREAKING-CHANGE: ",
        Increment::MAJOR
    );

    test_semantic_increment!(missing_space_causes_none, "fix:", Increment::NONE);
}
