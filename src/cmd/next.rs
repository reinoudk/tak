use clap::{ArgEnum, Parser};

use crate::error::Result;
use crate::git::SemanticRepository;
use crate::increment::Increment;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum IncrementArg {
    Patch,
    Minor,
    Major,
    Auto,
}

/// Determine the next version
#[derive(Parser, Debug)]
pub struct NextOpts {
    /// The type of version increment to use
    #[clap(arg_enum, default_value_t = IncrementArg::Auto)]
    increment: IncrementArg,
    /// Don't use the 'v' prefix
    #[clap(long, short)]
    no_prefix: bool,
    /// Write back the tag
    #[clap(long, short)]
    write: bool,
}

pub fn exec(next: &NextOpts) -> Result<()> {
    let prefix = if next.no_prefix { "" } else { "v" };
    let repo = SemanticRepository::open_with_prefix(prefix)?;

    let new_version = match next.increment {
        IncrementArg::Major => repo.next_version(Increment::Major),
        IncrementArg::Minor => repo.next_version(Increment::Minor),
        IncrementArg::Patch => repo.next_version(Increment::Patch),
        IncrementArg::Auto => repo.automatic_next_version(),
    }?;

    if next.write {
        repo.write_version(prefix, &new_version)?;
    }

    println!("{}{}", prefix, new_version);
    Ok(())
}
