use clap::{App, AppSettings, Arg};
use commands;
use config::Config;
use failure::Error;

pub fn main() -> Result<(), Error> {
    let args = cli().get_matches();

    let server_url = args.value_of("SERVER").unwrap();
    let master_key = args.value_of("master-key").unwrap();
    let config = Config::new(server_url, master_key);

    let (cmd, args) = match args.subcommand() {
        (cmd, Some(args)) => (cmd, args),
        _ => {
            return Ok(());
        }
    };

    if let Some(exec) = commands::cmd_exec(cmd) {
        return exec(&config, args);
    }

    Ok(())
}

fn cli() -> App<'static, 'static> {
    let app = App::new("Kotori CLI")
        .settings(&[AppSettings::SubcommandRequiredElseHelp, AppSettings::VersionlessSubcommands])
        .version("0.1.0")
        .arg(Arg::with_name("master-key")
            .help("Sets master key")
            .long("master-key")
            .required(true)
            .value_name("key")
            .takes_value(true))
        .arg(Arg::with_name("SERVER")
            .help("Kotori server endpoint")
            .required(true))
        .subcommands(commands::cmd_list());

    app
}
