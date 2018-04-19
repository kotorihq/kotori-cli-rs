use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use commands::kotori_group_command::KotoriGroupCommand;
use commands::project_create_upsert::ProjectCreateCommand;
use commands::project_delete::ProjectDeleteCommand;
use commands::project_list::ProjectListCommand;
use config::Config;
use failure::Error;

pub struct ProjectGroupCommand;

impl KotoriGroupCommand for ProjectGroupCommand {
    fn group_cli() -> App<'static, 'static> {
        SubCommand::with_name("project")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Manage projects")
            .subcommands(Self::cli())
    }

    fn cli() -> Vec<App<'static, 'static>> {
        vec![
            ProjectListCommand::cli(),
            ProjectCreateCommand::cli(),
            ProjectDeleteCommand::cli(),
        ]
    }

    fn cmd_exec(&self, subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
        let f = match subcmd {
            "list" => ProjectListCommand::exec,
            "create" => ProjectCreateCommand::exec,
            "delete" => ProjectDeleteCommand::exec,
            _ => {
                return None;
            }
        };

        Some(f)
    }
}