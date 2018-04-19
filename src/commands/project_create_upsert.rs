use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::{Response, StatusCode};
use std::collections::HashMap;
use url::Url;

#[derive(Deserialize, Debug)]
struct ProjectCreate {
    id: String,
    url: String,
}

pub struct ProjectCreateCommand;

impl ProjectCreateCommand {
    fn handle_success_response(response: &mut Response) -> Result<(), Error> {
        match response.status() {
            StatusCode::Ok => {
                println!("Project updated.");
            }

            StatusCode::Created => {
                let project_create: ProjectCreate = response.json()?;
                println!("Project created with ID: {}", project_create.id);
            }

            status_code => {
                panic!("Unknown response: {:?}", status_code);
            }
        }

        Ok(())
    }
}

impl KotoriCommand for ProjectCreateCommand {
    fn cli() -> App<'static, 'static> {
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

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let mut params = HashMap::new();
        params.insert("name", args.value_of("NAME").unwrap());

        let (url, method) = match args.value_of("with-id") {
            None => {
                let url = Url::parse(&config.server_url)?.join("/api/projects")?;
                let method = Method::Post;
                (url, method)
            }

            Some(id) => {
                let url = Url::parse(&config.server_url)?.join("/api/projects/")?.join(id)?;
                let method = Method::Put;
                (url, method)
            }
        };

        return super::command_base::do_request(config, method, &url, Some(&params),
                                               &ProjectCreateCommand::handle_success_response, None);
    }
}
