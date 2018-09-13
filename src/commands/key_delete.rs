use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;

#[derive(Deserialize, Debug)]
struct KeyCreate {
    id: String,
}

pub struct KeyDeleteCommand;

impl KeyDeleteCommand {
    fn handle_success_response(_response: &mut Response) -> Result<(), Error> {
        println!("Key deleted.");
        Ok(())
    }
}

impl KotoriCommand for KeyDeleteCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("delete")
            .about("Delete a project key")
            .arg(Arg::with_name("PROJECT ID")
                .help("Project ID")
                .required(true))
            .arg(Arg::with_name("KEY ID")
                .help("Key ID")
                .required(true))
    }

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let project_id = args.value_of("PROJECT ID").unwrap();
        let key_id = args.value_of("KEY ID").unwrap();
        let url = config.get_server_url()?.join(&format!("/api/projects/{}/project-keys/{}", project_id, key_id))?;

        return super::command_base::do_request(config, Method::Delete, &url, None,
                                               &KeyDeleteCommand::handle_success_response, None);
    }
}
