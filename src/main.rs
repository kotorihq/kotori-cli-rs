extern crate clap;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

use clap::{App, AppSettings, Arg, SubCommand};
use reqwest::{Client, StatusCode};
use reqwest::header::UserAgent;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
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

#[derive(Deserialize, Debug)]
struct ProjectCreate {
    id: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    message: String
}

fn project_list(server: &str, master_key: &str) -> Result<(), Box<Error>> {
    println!("Using Kotori endpoint: {}", server);

    let url = Url::parse(server)?.join("/api/projects")?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("kotori-cli"))
        .header(XMasterKey(master_key.to_owned()))
        .send()?;

    return handle_response(&mut response, &project_list_handle_response);
}

fn project_list_handle_response(response: &mut reqwest::Response) -> Result<(), Box<Error>> {
    let project_list: ProjectList = response.json()?;

    println!("Total count of projects: {}", project_list.count);

    println!("{:<25}{}", "ID", "PROJECT NAME");
    for project in &project_list.projects {
        println!("{:<25}{}", project.id, project.name);
    }

    Ok(())
}

fn project_create(name: &str, server: &str, master_key: &str) -> Result<(), Box<Error>> {
    println!("Using Kotori endpoint: {}", server);

    let url = Url::parse(server)?.join("/api/projects")?;

    let mut params = HashMap::new();
    params.insert("name", name);

    let mut response = Client::new()
        .post(url)
        .header(UserAgent::new("kotori-cli"))
        .header(XMasterKey(master_key.to_owned()))
        .json(&params)
        .send()?;

    return handle_response(&mut response, &project_create_handle_response);
}

fn project_create_handle_response(response: &mut reqwest::Response) -> Result<(), Box<Error>> {
    let project_create: ProjectCreate = response.json()?;
    println!("Project created with ID: {}", project_create.id);

    Ok(())
}

fn handle_response(response: &mut reqwest::Response, f: &Fn(&mut reqwest::Response) -> Result<(), Box<Error>>) -> Result<(), Box<Error>> {
    match response.status() {
        StatusCode::Ok |
        StatusCode::Created => {
            return f(response);
        }

        StatusCode::Unauthorized |
        StatusCode::Forbidden |
        StatusCode::InternalServerError => {
            let error_response: ErrorResponse = response.json()?;
            println!("Error: {}", error_response.message);
        }

        status_code => {
            panic!("Unknown response: {:?}", status_code);
        }
    }

    Ok(())
}

fn dispatch(matches: &clap::ArgMatches) -> Result<(), Box<Error>> {
    match matches.subcommand() {
        ("projects", Some(projects_matches)) => {
            let server = projects_matches.value_of("SERVER").unwrap();
            let master_key = projects_matches.value_of("master-key").unwrap();

            return project_list(server, master_key);
        }

        ("project", Some(project_matches)) => {
            if let Some(matches) = project_matches.subcommand_matches("create") {
                let name = matches.value_of("NAME").unwrap();
                let server = matches.value_of("SERVER").unwrap();
                let master_key = matches.value_of("master-key").unwrap();

                return project_create(name, server, master_key);
            }

            Ok(())
        }

        _ => { panic!("Should not have been called."); }
    }
}

fn main() {
    let matches = App::new("Kotori CLI")
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::VersionlessSubcommands])
        .version("0.1.0")
        .subcommand(SubCommand::with_name("projects")
            .about("Show list of projects")
            .arg(Arg::with_name("master-key")
                .help("Sets master key")
                .long("master-key")
                .required(true)
                .value_name("key")
                .takes_value(true))
            .arg(Arg::with_name("SERVER")
                .help("Kotori server endpoint")
                .required(true)))
        .subcommand(SubCommand::with_name("project")
            .setting(AppSettings::SubcommandRequiredElseHelp)
            .about("Manage projects")
            .subcommand(SubCommand::with_name("create")
                .about("Create a new project")
                .arg(Arg::with_name("master-key")
                    .help("Sets master key")
                    .long("master-key")
                    .required(true)
                    .value_name("key")
                    .takes_value(true))
                .arg(Arg::with_name("SERVER")
                    .help("Kotori server endpoint")
                    .required(true)
                    .index(1))
                .arg(Arg::with_name("NAME")
                    .help("Project name")
                    .required(true)
                    .index(2))))
        .get_matches();

    match dispatch(&matches) {
        Ok(_) => {}
        Err(e) => panic!("error: {:?}", e)
    };
}
