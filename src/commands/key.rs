use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::key_list;
use config::Config;
use failure::Error;
use commands::kotori_group_command::KotoriGroupCommand;

pub struct KeyGroupCommand;

impl KotoriGroupCommand for KeyGroupCommand {
    fn group_cmd_cli() -> App<'static, 'static> {
        SubCommand::with_name("key")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Manage keys")
            .subcommands(Self::cmd_cli())
    }

    fn cmd_cli() -> Vec<App<'static, 'static>> {
        vec![
            key_list::cli(),
        ]
    }

    fn cmd_exec(&self, subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
        let f = match subcmd {
            "list" => key_list::exec,
            _ => {
                return None;
            }
        };

        Some(f)
    }
}