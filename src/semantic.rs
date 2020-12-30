use git2::{Commit, Repository};
use regex::Regex;
use semver::Version;
use std::cmp;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Increment {
    NONE,
    PATCH,
    MINOR,
    MAJOR,
}

fn max_semantic_increment(messages: Vec<&str>) -> Increment {
    let increment = messages.into_iter().fold(Increment::NONE, |acc, message| {
        let increment = semantic_increment(message);

        // return the biggest increment type
        return cmp::max(acc, increment);
    });
    increment
}

fn semantic_increment(message: &str) -> Increment {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
          ^(?P<type>feat|fix|BREAKING\ CHANGE) # Type
          (?:\((?P<scope>.+)\))? # Scope (optional, without surrounding '()')
          (?P<breaking>!)? # Breaking change indicator (optional)
          (?::\ ) # Mandatory colon and space
          (?P<description>.+) # Short commit description
          ( # Ensure that optional commit body does not gobble up everything
            (?:(?s:.*))? # Commit body (optional)
            (?P<breaking_footer>(:?BREAKING-CHANGE|BREAKING\ CHANGE):\ ) # Breaking change in footer (optional, without surrounding '()')
          )?"
        )
        .unwrap();
    }

    let mut increment = Increment::NONE;

    if let Some(caps) = RE.captures(message) {
        if caps.name("breaking").is_some() || caps.name("breaking_footer").is_some() {
            // Increase major on breaking change
            increment = Increment::MAJOR;
        } else {
            // Use the value of type to determine increment
            increment = match caps.name("type") {
                Some(m) if m.as_str() == "fix" => Increment::PATCH,
                Some(m) if m.as_str() == "feat" => Increment::MINOR,
                Some(m) if m.as_str() == "BREAKING CHANGE" => Increment::MAJOR,
                None | _ => Increment::NONE,
            }
        }
    }
    increment
}

fn commits_since_version(repo: &Repository, version: Version) -> Result<Vec<Commit>, git2::Error> {
    let tag = format!("v{}", version);

    let mut walk = repo.revwalk().unwrap();
    walk.push_range(&format!("{}..HEAD", tag))?;
    let commits: Vec<Commit> = walk
        .filter_map(Result::ok)
        .map(|oid| {
            return repo.find_commit(oid).unwrap();
        })
        .collect();

    Ok(commits)
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
        breaking_change_results_in_major,
        "BREAKING CHANGE: description\n",
        Increment::MAJOR
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
