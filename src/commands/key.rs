use clap::{App, AppSettings, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use commands::key_list::KeyListCommand;
use commands::kotori_group_command::KotoriGroupCommand;
use config::Config;
use failure::Error;

pub struct KeyGroupCommand;

impl KotoriGroupCommand for KeyGroupCommand {
    fn group_cli() -> App<'static, 'static> {
        SubCommand::with_name("key")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Manage keys")
            .subcommands(Self::cli())
    }

    fn cli() -> Vec<App<'static, 'static>> {
        vec![
            KeyListCommand::cli(),
        ]
    }

    fn cmd_exec(&self, subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
        let f = match subcmd {
            "list" => KeyListCommand::exec,
            _ => {
                return None;
            }
        };

        Some(f)
    }
}