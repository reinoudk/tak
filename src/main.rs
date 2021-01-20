use clap::{crate_version, App, AppSettings};

use cmd::increment;

mod cmd;

fn main() -> Result<(), String> {
    // set up basic cli arguments
    let app = App::new("tak")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .about("A Git tag helper.");

    // add increment command
    let app = app.subcommand(increment::cmd());

    let matches = app.get_matches();
    let out = &mut std::io::stdout();

    match matches.subcommand() {
        (increment::CMD_NAME, Some(sub_matches)) => increment::exec(sub_matches, out),
        _ => Ok(()),
    }
    .map_err(|err| err.to_string())
}
