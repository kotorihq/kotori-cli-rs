use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;
use url::Url;

#[derive(Deserialize, Debug)]
struct KeyList {
    count: i64,

    #[serde(rename = "items")]
    keys: Vec<Key>,
}

#[derive(Deserialize, Debug)]
struct Key {
    key: String,

    #[serde(rename = "isReadonly")]
    read_only: bool,
}

pub struct KeyListCommand;

impl KeyListCommand {
    fn handle_success_response(response: &mut Response) -> Result<(), Error> {
        let key_list: KeyList = response.json()?;

        println!("Total count of keys: {}", key_list.count);

        println!("{:<25}{}", "KEY", "READ-ONLY");
        for key in &key_list.keys {
            println!("{:<25}{}", key.key, if key.read_only { "yes" } else { "no" });
        }

        Ok(())
    }
}

impl KotoriCommand for KeyListCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("list")
            .about("Show project keys")
            .arg(Arg::with_name("PROJECT ID")
                .help("Project ID")
                .required(true))
    }

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let project_id = args.value_of("PROJECT ID").unwrap();
        let url = Url::parse(&format!("{}/api/projects/{}/project-keys", &config.server_url, project_id))?;

        return super::command_base::do_request(config, Method::Get, &url, None,
                                               &KeyListCommand::handle_success_response, None);
    }
}
