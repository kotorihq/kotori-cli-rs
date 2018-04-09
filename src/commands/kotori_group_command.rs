use clap::{App, ArgMatches};
use config::Config;
use failure::Error;

pub trait KotoriGroupCommand {
    fn group_cmd_cli() -> App<'static, 'static> where Self: Sized;

    fn cmd_cli() -> Vec<App<'static, 'static>> where Self: Sized;

    fn group_exec(&self, config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let (cmd, args) = match args.subcommand() {
            (cmd, Some(args)) => (cmd, args),
            _ => {
                return Ok(());
            }
        };

        if let Some(exec) = self.cmd_exec(cmd) {
            return exec(config, args);
        }

        Ok(())
    }

    fn cmd_exec(&self, subcmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>>;
}