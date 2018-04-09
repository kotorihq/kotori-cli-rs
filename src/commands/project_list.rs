use clap::{App, ArgMatches, SubCommand};
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;
use url::Url;

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

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("list")
        .about("Show list of projects")
}

pub fn exec(config: &Config, _args: &ArgMatches) -> Result<(), Error> {
    let url = Url::parse(&config.server_url)?.join("/api/projects")?;

    return super::command_base::do_request(config, Method::Get, &url, None,
                                           &handle_success_response, None);
}

fn handle_success_response(response: &mut Response) -> Result<(), Error> {
    let project_list: ProjectList = response.json()?;

    println!("Total count of projects: {}", project_list.count);

    println!("{:<25}{}", "ID", "PROJECT NAME");
    for project in &project_list.projects {
        println!("{:<25}{}", project.id, project.name);
    }

    Ok(())
}
