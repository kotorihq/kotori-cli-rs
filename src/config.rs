use failure::Error;
use url::ParseError;
use url::Url;

#[derive(Debug)]
pub struct Config {
    server_url: String,
    pub master_key: String,
}

impl Config {
    pub fn new(server_url: &str, master_key: &str) -> Config {
        Config {
            server_url: server_url.to_owned(),
            master_key: master_key.to_owned(),
        }
    }

    pub fn get_server_url(&self) -> Result<Url, Error> {
        match Url::parse(&self.server_url) {
            Ok(url) => {
                Ok(url)
            }

            Err(e) => {
                if e == ParseError::RelativeUrlWithoutBase {
                    let url = Url::parse(&format!("https://{}", &self.server_url))?;
                    return Ok(url);
                }

                bail!(e)
            }
        }
    }
}

