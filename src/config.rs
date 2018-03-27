#[derive(Debug)]
pub struct Config {
    pub server_url: String,
    pub master_key: String,
}

impl Config {
    pub fn new(server_url: &str, master_key: &str) -> Config {
        Config {
            server_url: server_url.to_owned(),
            master_key: master_key.to_owned(),
        }
    }
}
