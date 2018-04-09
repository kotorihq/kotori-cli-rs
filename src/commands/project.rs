use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::{project_create_upsert, project_delete, project_list};
use commands::kotori_group_command::KotoriGroupCommand;
use config::Config;
use failure::Error;

pub struct ProjectGroupCommand;

impl KotoriGroupCommand for ProjectGroupCommand {
    fn group_cmd_cli() -> App<'static, 'static> {
        SubCommand::with_name("project")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Manage projects")
            .subcommands(Self::cmd_cli())
    }

    fn cmd_cli() -> Vec<App<'static, 'static>> {
        vec![
            project_list::cli(),
            project_create_upsert::cli(),
            project_delete::cli(),
        ]
    }

    fn cmd_exec(&self, subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
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
}