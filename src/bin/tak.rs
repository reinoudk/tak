use clap::Parser;

use tak::cmd::next::{self, NextOpts};

/// tak - a Git tagging helper
#[derive(Parser)]
#[clap(version)]
struct Opts {
    #[clap(subcommand)]
    sub_cmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Next(NextOpts),
}

fn main() {
    let opts: Opts = Opts::parse();

    if let Err(err) = match &opts.sub_cmd {
        SubCommand::Next(next_opts) => next::exec(next_opts),
    } {
        eprintln!("Error: {}", err.to_string());
    }
}
