use clap::App;
use clap::ArgMatches;
use config::Config;
use failure::Error;

mod command_base;

pub mod project;
pub mod project_list;
pub mod project_create_upsert;
pub mod project_delete;

pub mod key;
pub mod key_list;

pub fn cmd_list() -> Vec<App<'static, 'static>> {
    vec![
        project::cli(),
        key::cli(),
    ]
}

pub fn cmd_exec(cmd: &str) -> Option<fn(&Config, &ArgMatches) -> Result<(), Error>> {
    let f = match cmd {
        "project" => project::exec,
        "key" => key::exec,
        _ => {
            return None;
        }
    };

    Some(f)
}