use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::key_list;
use config::Config;
use failure::Error;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("key")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Manage keys")
        .subcommands(subcmd_list())
}

pub fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
    let (cmd, args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Ok(());
        }
    };

    if let Some(exec) = subcmd_exec(cmd) {
        return exec(config, args);
    }

    Ok(())
}

fn subcmd_list() -> Vec<App<'static, 'static>> {
    vec![
        key_list::cli(),
    ]
}

fn subcmd_exec(subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
    let f = match subcmd {
        "list" => key_list::exec,
        _ => {
            return None;
        }
    };


    Some(f)
}