use config::Config;
use failure::Error;
use reqwest::{Client, Method, Response};
use reqwest::header::UserAgent;
use reqwest::StatusCode;
use std::collections::HashMap;
use url::Url;

header! { (XMasterKey, "x-master-key") => [String] }

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String
}

pub fn do_request(config: &Config, method: Method, url: &Url, params: Option<&HashMap<&str, &str>>,
                  f_success: &Fn(&mut Response) -> Result<(), Error>,
                  f_error: Option<&Fn(&mut Response) -> Result<(), Error>>) -> Result<(), Error> {
    let mut request = Client::new().request(method, url.clone());
    request.header(UserAgent::new("kotori-cli")).header(XMasterKey(config.master_key.to_owned()));

    if let Some(params) = params {
        request.json(params);
    }

    let mut response = request.send()?;

    return handle_response(&mut response, &f_success, &f_error);
}

fn handle_response(response: &mut Response,
                   f_success: &Fn(&mut Response) -> Result<(), Error>,
                   f_error: &Option<&Fn(&mut Response) -> Result<(), Error>>) -> Result<(), Error> {
    if response.status().is_success() {
        return f_success(response);
    }

    if let &Some(f_err) = f_error {
        return f_err(response);
    }

    return default_handle_error_response(response);
}

fn default_handle_error_response(response: &mut Response) -> Result<(), Error> {
    match response.status() {
        StatusCode::Unauthorized |
        StatusCode::Forbidden |
        StatusCode::NotFound |
        StatusCode::InternalServerError |
        StatusCode::BadRequest => {
            let error_response: ErrorResponse = response.json()?;
            println!("Error: {}", error_response.message);
        }

        status_code => {
            panic!("Unknown response: {:?}", status_code);
        }
    }

    Ok(())
}