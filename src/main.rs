extern crate clap;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

use clap::{App, AppSettings, Arg, SubCommand};
use reqwest::Client;
use reqwest::header::UserAgent;
use std::error::Error;
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

fn project_list(server: &str, master_key: &str) -> Result<(), Box<Error>> {
    println!("Using Kotori endpoint: {}", server);

    let url = Url::parse(server)?.join("/api/projects")?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("kotori-cli"))
        .header(XMasterKey(master_key.to_owned()))
        .send()?;

    let project_list: ProjectList = response.json()?;

    println!("Total count of projects: {}", project_list.count);

    println!("ID\t\t\tPROJECT NAME");
    for project in &project_list.projects {
        println!("{}\t\t\t{}", project.id, project.name);
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
        .get_matches();

    match dispatch(&matches) {
        Ok(_) => {}
        Err(e) => panic!("error: {:?}", e)
    };
}
