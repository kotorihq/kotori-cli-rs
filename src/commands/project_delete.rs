use clap::{App, Arg, ArgMatches, SubCommand};
use commands::command_base::ErrorResponse;
use failure::Error;
use reqwest::{Client, Response, StatusCode};
use reqwest::header::UserAgent;
use std::collections::HashMap;
use url::Url;

header! { (XMasterKey, "x-master-key") => [String] }

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("delete")
        .about("Delete a project")
        .arg(Arg::with_name("PROJECT ID")
            .help("Project ID")
            .required(true))
}

pub fn exec(args: &ArgMatches, server: &str, master_key: &str) -> Result<(), Error> {
    let project_id = args.value_of("PROJECT ID").unwrap();

    let url = Url::parse(server)?.join("/api/projects/")?.join(project_id)?;

    let mut params = HashMap::new();
    params.insert("name", project_id);

    let mut response = Client::new()
        .delete(url)
        .header(UserAgent::new("kotori-cli"))
        .header(XMasterKey(master_key.to_owned()))
        .send()?;

    return handle_response(&mut response, &project_delete_handle_response);
}

fn project_delete_handle_response(_response: &mut Response) -> Result<(), Error> {
    println!("Project deleted.");
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
