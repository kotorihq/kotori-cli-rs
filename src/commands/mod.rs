use clap::App;
use clap::ArgMatches;
use failure::Error;

mod command_base;

pub mod project;
pub mod project_list;
pub mod project_create_upsert;
pub mod project_delete;

pub fn cmd_list() -> Vec<App<'static, 'static>> {
    vec![
        project::cli(),
    ]
}

pub fn cmd_exec(cmd: &str) -> Option<fn(&ArgMatches) -> Result<(), Error>> {
    let f = match cmd {
        "project" => project::exec,
        _ => {
            return None;
        }
    };

    Some(f)
}