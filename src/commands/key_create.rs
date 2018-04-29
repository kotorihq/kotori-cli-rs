use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;
use url::Url;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct KeyCreate {
    id: String,
}

pub struct KeyCreateCommand;

impl KeyCreateCommand {
    fn handle_success_response(response: &mut Response) -> Result<(), Error> {
        let key_create: KeyCreate = response.json()?;
        println!("Key created: {}", key_create.id);

        Ok(())
    }
}

impl KotoriCommand for KeyCreateCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("create")
            .about("Show project keys")
            .arg(Arg::with_name("PROJECT ID")
                .help("Project ID")
                .required(true))
            .arg(Arg::with_name("readonly")
                .help("Key will have read-only rights")
                .long("readonly")
                .required(false)
                .takes_value(false))
    }

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let project_id = args.value_of("PROJECT ID").unwrap();
        let url = Url::parse(&format!("{}/api/projects/{}/project-keys", &config.server_url, project_id))?;

        let is_readonly = args.is_present("readonly").to_string();

        let mut params = HashMap::new();
        params.insert("isReadonly", is_readonly.as_str());

        return super::command_base::do_request(config, Method::Post, &url, Some(&params),
                                               &KeyCreateCommand::handle_success_response, None);
    }
}
