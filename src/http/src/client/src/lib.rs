use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use reqwest::{header};

#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub fn list_files(endpoint_url: &str, authorization_token: &str, path: OsString) -> Result<ListFilesResponse, Error> {
    let url = "https://hub.blockstack.org/list-files";
    let rewrited_url = endpoint_url.to_string().replace("https://gaia.blockstack.org/hub", url);
    let client = reqwest::Client::new();
    let request = client.post(&rewrited_url)
        .header(header::AUTHORIZATION, format!("bearer {}", authorization_token))
        .send();

    let mut res = match request {
        Ok(res) => res,
        Err(_err) => {
            return Err(Error::Unknown);
        }
    };

    let payload: ListFilesResponse = match res.json() {
        Ok(payload) => payload,
        Err(_err) => {
            return Err(Error::Unknown);
        }
    };

    Ok(payload)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListFilesResponse {
    pub entries: Vec<String>,
    pub page: Option<u64>,
}
