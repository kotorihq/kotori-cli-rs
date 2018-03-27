use clap::{App, Arg, ArgMatches, SubCommand};
use commands::command_base::ErrorResponse;
use config::Config;
use failure::Error;
use reqwest::{Client, Response, StatusCode};
use reqwest::header::UserAgent;
use std::collections::HashMap;
use url::Url;

header! { (XMasterKey, "x-master-key") => [String] }

#[derive(Deserialize, Debug)]
struct ProjectCreate {
    id: String,
    url: String,
}

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("create")
        .about("Create a new project")
        .arg(Arg::with_name("NAME")
            .help("Project name")
            .required(true))
        .arg(Arg::with_name("with-id")
            .help("Sets project ID\nIf project with given ID already exists,\nupdate it instead")
            .long("with-id")
            .required(false)
            .value_name("id")
            .takes_value(true))
}

pub fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
    let name = args.value_of("NAME").unwrap();
    let mut params = HashMap::new();
    params.insert("name", name);

    let mut response = match args.value_of("with-id") {
        None => {
            Client::new()
                .post(Url::parse(&config.server_url)?.join("/api/projects")?)
                .header(UserAgent::new("kotori-cli"))
                .header(XMasterKey(config.master_key.to_owned()))
                .json(&params)
                .send()?
        }

        Some(id) => {
            Client::new()
                .put(Url::parse(&config.server_url)?.join("/api/projects/")?.join(id)?)
                .header(UserAgent::new("kotori-cli"))
                .header(XMasterKey(config.master_key.to_owned()))
                .json(&params)
                .send()?
        }
    };

    return handle_response(&mut response, &project_create_upsert_handle_response);
}

fn project_create_upsert_handle_response(response: &mut Response) -> Result<(), Error> {
    if response.status() == StatusCode::Ok {
        println!("Project updated.");
    } else if response.status() == StatusCode::Created {
        let project_create: ProjectCreate = response.json()?;
        println!("Project created with ID: {}", project_create.id);
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
