use clap::{ArgEnum, Parser};

use crate::error::Result;
use crate::git::SemanticRepository;
use crate::increment::Increment;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum IncrementArg {
    PATCH,
    MINOR,
    MAJOR,
    AUTO,
}

/// Determine the next version
#[derive(Parser, Debug)]
pub struct NextOpts {
    /// The type of version increment to use
    #[clap(arg_enum, default_value_t = IncrementArg::AUTO)]
    increment: IncrementArg,
    /// Don't use the 'v' prefix
    #[clap(long, short)]
    no_prefix: bool,
}

pub fn exec(next: &NextOpts) -> Result<()> {
    let prefix = if next.no_prefix { "" } else { "v" };
    let repo = SemanticRepository::open_with_prefix(prefix)?;

    let new_version = match next.increment {
        IncrementArg::MAJOR => repo.next_version(Increment::MAJOR),
        IncrementArg::MINOR => repo.next_version(Increment::MINOR),
        IncrementArg::PATCH => repo.next_version(Increment::PATCH),
        IncrementArg::AUTO => repo.automatic_next_version(),
    };

    println!("{}{}", prefix, new_version?.to_string());
    Ok(())
}
