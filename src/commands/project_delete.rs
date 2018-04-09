use clap::{App, Arg, ArgMatches, SubCommand};
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;
use url::Url;

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("delete")
        .about("Delete a project")
        .arg(Arg::with_name("PROJECT ID")
            .help("Project ID")
            .required(true))
}

pub fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
    let project_id = args.value_of("PROJECT ID").unwrap();
    let url = Url::parse(&config.server_url)?.join("/api/projects/")?.join(project_id)?;

    return super::command_base::do_request(config, Method::Delete, &url, None,
                                           &handle_success_response, None);
}

fn handle_success_response(_response: &mut Response) -> Result<(), Error> {
    println!("Project deleted.");
    Ok(())
}
