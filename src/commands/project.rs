use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::{project_create_upsert, project_delete, project_list};
use config::Config;
use failure::Error;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("project")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .about("Manage projects")
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
        project_list::cli(),
        project_create_upsert::cli(),
        project_delete::cli(),
    ]
}

fn subcmd_exec(subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
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