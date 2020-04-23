use clap::{App, AppSettings};
use std::io::Error;

mod increment;

fn main() -> Result<(), Error> {
    // set up basic cli arguments
    let app = App::new("tak")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("A Git tag helper.");

    // add increment command
    let app = app.subcommand(increment::cmd());

    let matches = app.get_matches();
    let out = &mut std::io::stdout();

    match matches.subcommand() {
        (increment::CMD_NAME, Some(sub_matches)) => increment::exec(sub_matches, out),
        _ => Ok(()),
    }
}
