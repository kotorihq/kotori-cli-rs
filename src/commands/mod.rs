use clap::App;
use commands::key::KeyGroupCommand;
use commands::kotori_group_command::KotoriGroupCommand;
use commands::project::ProjectGroupCommand;

mod command_base;

pub mod kotori_group_command;

pub mod project;
pub mod project_list;
pub mod project_create_upsert;
pub mod project_delete;

pub mod key;
pub mod key_list;

pub fn cmd_list() -> Vec<App<'static, 'static>> {
    vec![
        ProjectGroupCommand::group_cli(),
        KeyGroupCommand::group_cli(),
    ]
}

pub fn cmd_exec(cmd: &str) -> Option<Box<KotoriGroupCommand>> {
    let f: Box<KotoriGroupCommand> = match cmd {
        "project" => Box::new(ProjectGroupCommand {}),
        "key" => Box::new(KeyGroupCommand {}),
        _ => {
            return None;
        }
    };

    Some(f)
}