use clap::{App, Arg, ArgMatches, SubCommand};
use commands::kotori_command::KotoriCommand;
use config::Config;
use failure::Error;
use hyper::Method;
use reqwest::Response;

pub struct ProjectDeleteCommand;

impl ProjectDeleteCommand {
    fn handle_success_response(_response: &mut Response) -> Result<(), Error> {
        println!("Project deleted.");
        Ok(())
    }
}

impl KotoriCommand for ProjectDeleteCommand {
    fn cli() -> App<'static, 'static> {
        SubCommand::with_name("delete")
            .about("Delete a project")
            .arg(Arg::with_name("PROJECT ID")
                .help("Project ID")
                .required(true))
    }

    fn exec(config: &Config, args: &ArgMatches) -> Result<(), Error> {
        let project_id = args.value_of("PROJECT ID").unwrap();
        let url = config.get_server_url()?.join("/api/projects/")?.join(project_id)?;

        return super::command_base::do_request(config, Method::Delete, &url, None,
                                               &ProjectDeleteCommand::handle_success_response, None);
    }
}
