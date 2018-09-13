use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::{Response, StatusCode};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct KeyCreate {
    id: String,
}

pub struct KeyCreateUpsertCommand;

impl KeyCreateUpsertCommand {
    fn handle_success_response(response: &mut Response) -> Result<(), Error> {
        match response.status() {
            StatusCode::Ok => {
                println!("Key updated.");
            }

            StatusCode::Created => {
                let key_create: KeyCreate = response.json()?;
                println!("Key created with ID: {}", key_create.id);
            }

            status_code => {
                panic!("Unknown response: {:?}", status_code);
            }
        }

        Ok(())
    }
}

impl KotoriCommand for KeyCreateUpsertCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("create")
            .about("Create a project key")
            .arg(Arg::with_name("PROJECT ID")
                .help("Project ID")
                .required(true))
            .arg(Arg::with_name("with-id")
                .help("Sets key ID\nIf key with given ID already exists,\nupdate it instead")
                .long("with-id")
                .required(false)
                .value_name("id")
                .takes_value(true))
            .arg(Arg::with_name("readonly")
                .help("Key will have read-only rights")
                .long("readonly")
                .required(false)
                .takes_value(false))
    }

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let project_id = args.value_of("PROJECT ID").unwrap();

        let is_readonly = args.is_present("readonly").to_string();
        let mut params = HashMap::new();
        params.insert("isReadonly", is_readonly.as_str());

        let (url, method) = match args.value_of("with-id") {
            None => {
                let url = config.get_server_url()?.join(&format!("/api/projects/{}/project-keys", project_id))?;
                let method = Method::Post;
                (url, method)
            }

            Some(key_id) => {
                let url = config.get_server_url()?.join(&format!("/api/projects/{}/project-keys/{}", project_id, key_id))?;
                let method = Method::Put;
                (url, method)
            }
        };

        return super::command_base::do_request(config, method, &url, Some(&params),
                                               &KeyCreateUpsertCommand::handle_success_response, None);
    }
}
