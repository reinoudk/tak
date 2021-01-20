use clap::{crate_version, App, AppSettings};

use cmd::next;

mod cmd;

fn main() {
    // set up basic cli arguments
    let app = App::new("tak")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about("A Git tag helper.");

    // add next command
    let app = app.subcommand(next::cmd());

    let matches = app.get_matches();

    if let Err(err) = match matches.subcommand() {
        (next::CMD_NAME, Some(sub_matches)) => next::exec(sub_matches),
        _ => Ok(()),
    } {
        eprintln!("Error: {}", err.to_string());
    }

}
