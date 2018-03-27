extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use commands::{project_create_upsert, project_delete, project_list};
use failure::Error;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Manage projects")
        .arg(Arg::with_name("master-key")
            .help("Sets master key")
            .long("master-key")
            .required(true)
            .value_name("key")
            .takes_value(true))
        .arg(Arg::with_name("SERVER")
            .help("Kotori server endpoint")
            .required(true))
        .subcommands(subcmd_list())
}

pub fn exec(args: &ArgMatches) -> Result<(), Error> {
    let server = args.value_of("SERVER").unwrap();
    let master_key = args.value_of("master-key").unwrap();

    let (cmd, args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Ok(());
        }
    };

    if let Some(exec) = subcmd_exec(cmd) {
        return exec(args, server, master_key);
    }

    Ok(())
}

fn subcmd_list() -> Vec<App<'static, 'static>> {
    vec![
        project_list::cli(),
        project_create_upsert::cli(),
        project_delete::cli(),
    ]
}

fn subcmd_exec(subcmd: &str) -> Option<fn(&ArgMatches, &str, &str) -> Result<(), Error>> {
    let f = match subcmd {
        "list" => project_list::exec,
        "create" => project_create_upsert::exec,
        "delete" => project_delete::exec,
        _ => {
            return None;
        }
    };


    Some(f)
}