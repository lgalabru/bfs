#![feature(async_await)]
#[macro_use] extern crate log;
extern crate env_logger;
use std::ffi::{OsStr, OsString};
use std::io;
use std::collections::HashMap;
use std::env;
use std::cmp;

use file_system::{FS, SyncEngine};
use tokio;
use crate::authenticator::LocalAuthenticator;

use bip39::{Mnemonic, Seed, Language};

use bfs_api::{get_names, get_user};

use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};
use bfs_auth::v1::{
    helpers:: {
        get_bip39_seed_from_mnemonic, 
        get_hardened_child_keypair,
        get_address_from_public_key
    },
    CreateAuthorizationToken,
    CreateAuthorizationRequestToken,
    types::AuthScope
};

use bfs_commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
};

mod file_system;
mod authenticator;
mod commands;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let prefix_path = "/";

    println!("Mnemonic (12 words):");
    let mut input = String::new();
    let mut phrase = match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(e) => { panic!() }
    };
    phrase.pop();

    let mnemonic = match Mnemonic::from_phrase(phrase, Language::English) {
        Ok(mnemonic) => mnemonic,
        Err(e) => { panic!() }
    };

    let bip39_seed = match get_bip39_seed_from_mnemonic(&phrase, "") {
        Ok(bip39_seed) => bip39_seed,
        Err(e) => { panic!() }
    };

    let (_, public_key) = match get_hardened_child_keypair(&bip39_seed, &[888, 0, 0]) {
        Ok(result) => result,
        Err(e) => { panic!() }
    };

    let address = match get_address_from_public_key(&public_key) {
        Ok(result) => result,
        Err(e) => { panic!() }
    };

    let name = match get_names(&address) {
        Ok(names) => names[0].clone(),  // todo(ludo): handle multiple identities
        Err(_) => panic!() 
    };

    let users = match get_user(&name) {
        Ok(users) => users, // todo(ludo): handle multiple profiles
        Err(_) => panic!() 
    };

    let user = users.get(&name).unwrap();
    let authorization_tokens: HashMap<String, String> = HashMap::new();
    let mut sync_engine = SyncEngine::new();

    for (app_domain, url) in user.profile.apps.iter() {
        let app_name = {
            let app_domain_striped: &str = if app_domain.starts_with("https://") {
                &app_domain[8..]
            } else if app_domain.starts_with("http://") {
                let comps: Vec<&str> = app_domain[7..].split(":").collect();
                comps[0]
            } else {
                panic!()
            };
            let comps: Vec<&str> = app_domain_striped.split(".").collect();
            OsString::from(comps.join("-"))
        };
        println!("Mounting {:?}", app_name);

        // file_map.new_directory(1, &app_name);

        let command = CreateAuthorizationRequestToken::new(
            app_domain.to_string(),
            app_domain.to_string(),
            app_domain.to_string(),
            "1.0".to_string(),
            vec![],    // todo(ludo): fill
            true,     // todo(ludo): fill
            true,     // todo(ludo): fill
        );

        let (authorization_request_token, transit_secret_key) = match command.run() {
            Ok(token) => token,
            Err(e) => panic!()
        };

        let command = CreateAuthorizationToken::new(
            bip39_seed.clone(),
            authorization_request_token,
            "".to_string(), // todo(ludo): fill gaia_challenge
            url.to_string(),
            0
        );

        let authorization_token = match command.run() {
            Ok(token) => token,
            Err(e) => panic!()
        };

        sync_engine.register_endpoint(app_name, url.to_string(), authorization_token);

        // authorization_tokens.insert(app_name.to_string(), authorization_token);
    }

    let filesystem = FS::new(sync_engine);

    fuse::mount(filesystem, &mountpoint, &options).unwrap();


    Ok(())
}