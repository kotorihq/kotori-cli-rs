extern crate clap;

use clap::{App, AppSettings};
use failure::Error;

use super::commands;

pub fn main() -> Result<(), Error> {
    let args = cli().get_matches();

    let (cmd, args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Ok(());
        }
    };

    if let Some(exec) = commands::cmd_exec(cmd) {
        return exec(args);
    }

    Ok(())
}

fn cli() -> App<'static, 'static> {
    let app = App::new("Kotori CLI")
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::VersionlessSubcommands])
        .version("0.1.0")
        .subcommands(commands::cmd_list());

    app
}
