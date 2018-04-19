use clap::{App, ArgMatches};
use config::Config;
use failure::Error;

pub trait KotoriCommand {
    fn cli() -> App<'static, 'static> where Self: Sized;

    fn exec(config: &Config, _args: &ArgMatches) -> Result<(), Error>;
}