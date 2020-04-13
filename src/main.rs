use clap::{App, AppSettings};

mod increment;

fn main() {
    // set up basic cli arguments
    let app = App::new("tak")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .about("A Git tag helper.");

    // add increment command
    let app = app.subcommand(increment::cmd());

    let matches = app.get_matches();

    match matches.subcommand() {
        (increment::CMD_NAME, Some(sub_matches)) => increment::exec(sub_matches),
        _ => {}
    }
}
