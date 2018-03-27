use clap::{App, ArgMatches, SubCommand};
use commands::command_base::ErrorResponse;
use failure::Error;
use reqwest::{Client, Response, StatusCode};
use reqwest::header::UserAgent;
use url::Url;

header! { (XMasterKey, "x-master-key") => [String] }

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

pub fn exec(_args: &ArgMatches, server: &str, master_key: &str) -> Result<(), Error> {
    let url = Url::parse(server)?.join("/api/projects")?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("kotori-cli"))
        .header(XMasterKey(master_key.to_owned()))
        .send()?;

    return handle_response(&mut response, &project_list_handle_response);
}

fn project_list_handle_response(response: &mut Response) -> Result<(), Error> {
    let project_list: ProjectList = response.json()?;

    println!("Total count of projects: {}", project_list.count);

    println!("{:<25}{}", "ID", "PROJECT NAME");
    for project in &project_list.projects {
        println!("{:<25}{}", project.id, project.name);
    }

    Ok(())
}

fn handle_response(response: &mut Response, f: &Fn(&mut Response) -> Result<(), Error>) -> Result<(), Error> {
    match response.status() {
        StatusCode::Ok |
        StatusCode::Created |
        StatusCode::NoContent => {
            return f(response);
        }

        StatusCode::Unauthorized |
        StatusCode::Forbidden |
        StatusCode::NotFound |
        StatusCode::InternalServerError |
        StatusCode::BadRequest => {
            let error_response: ErrorResponse = response.json()?;
            println!("Error: {}", error_response.message);
        }

        status_code => {
            panic!("Unknown response: {:?}", status_code);
        }
    }

    Ok(())
}
