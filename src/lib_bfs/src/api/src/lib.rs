use serde::{Deserialize, Serialize};
use reqwest;
use std::collections::HashMap;

pub enum Error {
    Unknown,
}

pub fn get_names(address: &str) -> Result<Vec<String>, Error> {
    let url = format!("https://core.blockstack.org/v1/addresses/bitcoin/{}", address);
    let payload: HashMap<String, Vec<String>> = reqwest::get(&url).unwrap().json().unwrap();
    // todo(ludo): handle these unwrap()
    let names = payload.get("names").unwrap();
    Ok((names.to_vec()))
}

pub fn get_user(name: &str) -> Result<HashMap<String, User>, Error> {
    let url = format!("https://core.blockstack.org/v1/users/{}", name);
    let resp: HashMap<String, User> = reqwest::get(&url).unwrap().json().unwrap();
    // todo(ludo): handle these unwrap()
    let user = &*resp.get(name).unwrap();
    Ok((resp))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub owner_address: String,
    pub profile: Profile
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub apps: HashMap<String, String>,
    pub api: Api
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Api {
    #[serde(rename = "gaiaHubConfig")]
    pub gaia_hub_config: GaiaHubConfig,
    #[serde(rename = "gaiaHubUrl")]
    pub gaia_hub_url: String
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct GaiaHubConfig {
    pub url_prefix: String
} 

