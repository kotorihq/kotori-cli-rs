use clap::{App, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;

#[derive(Deserialize, Debug)]
struct ProjectList {
    count: i64,

    #[serde(rename = "items")]
    projects: Vec<Project>,
}

#[derive(Deserialize, Debug)]
struct Project {
    id: String,
    name: String,
}

pub struct ProjectListCommand;

impl ProjectListCommand {
    fn handle_success_response(response: &mut Response) -> Result<(), Error> {
        let project_list: ProjectList = response.json()?;

        println!("Total count of projects: {}", project_list.count);

        println!("{:<25}{}", "ID", "PROJECT NAME");
        for project in &project_list.projects {
            println!("{:<25}{}", project.id, project.name);
        }

        Ok(())
    }
}

impl KotoriCommand for ProjectListCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("list")
            .about("Show list of projects")
    }

    fn exec(config: &Config, _args: &ArgMatches) -> Result<(), Error> {
        let url = config.get_server_url()?.join("/api/projects")?;

        return super::command_base::do_request(config, Method::Get, &url, None,
                                               &ProjectListCommand::handle_success_response, None);
    }
}
