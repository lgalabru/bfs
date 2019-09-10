#![feature(async_await)]
extern crate env_logger;
use std::ffi::{OsStr, OsString};
use std::io;
use std::env;
use file_system::{FS, SyncEngine};

use bip39::{Mnemonic, Language};

use blockstack::bns::{get_identities, get_user};
use blockstack::auth::v1::{
    helpers:: {
        get_bip39_seed_from_mnemonic, 
        get_hardened_child_keypair,
        get_address_from_public_key
    },
    tokens:: {
        CreateAuthorizationToken,
        CreateAuthorizationRequestToken,
        VerifyAuthorizationToken,
        CreateHubToken
    }
};

mod file_system;
mod authenticator;
mod commands;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["fsname=Gaia"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    println!("Mnemonic (12 words):");
    let mut input = String::new();
    let mut phrase = match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(_) => { panic!() }
    };
    phrase.pop();

    let _mnemonic = match Mnemonic::from_phrase(phrase, Language::English) {
        Ok(mnemonic) => mnemonic,
        Err(_) => { panic!() }
    };

    let bip39_seed = match get_bip39_seed_from_mnemonic(&phrase, "") {
        Ok(bip39_seed) => bip39_seed,
        Err(_) => { panic!() }
    };

    let (_, public_key) = match get_hardened_child_keypair(&bip39_seed, &[888, 0, 0]) {
        Ok(result) => result,
        Err(_) => { panic!() }
    };

    let address = match get_address_from_public_key(&public_key) {
        Ok(result) => result,
        Err(_) => { panic!() }
    };

    println!("Retrieving identities...");
    let identity = match get_identities(&address) {
        Ok(identities) => {

            for (i, identity) in identities.iter().enumerate() {
                println!("[{}] {:?}", i, identity);
            }
            
            if identities.len() == 1 {
                identities[0].clone()
            } else {
                println!("Identity to load:");
                let mut input = String::new();
                let mut identity_index = match io::stdin().read_line(&mut input) {
                    Ok(_) => input,
                    Err(_) => { panic!() }
                };
                identity_index.pop();
                let identity_index = identity_index.parse::<usize>().unwrap();
                identities[identity_index].clone()
            }
        },
        Err(_) => panic!() 
    };
    println!("Loading {:?}", identity);

    let users = match get_user(&identity) {
        Ok(users) => users,
        Err(_) => panic!() 
    };

    let user = users.get(&identity).unwrap();
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
        println!("Caching {:?}", app_name);

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
            Err(_) => panic!()
        };

        // let challenge = "[\"gaiahub\",\"0\",\"hub\",\"blockstack_storage_please_sign\"]".to_string();
        let challenge = "[\"gaiahub\",\"0\",\"storage2.blockstack.org\",\"blockstack_storage_please_sign\"]".to_string();
        let command = CreateAuthorizationToken::new(
            bip39_seed.clone(),
            authorization_request_token,
            challenge.clone(), // todo(ludo): fill gaia_challenge
            url.to_string(),
            0
        );

        let authorization_token = match command.run() {
            Ok(token) => token,
            Err(_) => panic!()
        };

        let mut command = VerifyAuthorizationToken::new(
            authorization_token.clone(),
            transit_secret_key
        );

        let app_secret_key = match command.run() {
            Ok(token) => token,
            Err(_) => panic!()
        };

        let command = CreateHubToken::new(
            app_secret_key.clone(),
            challenge.clone(), // todo(ludo): fill gaia_challenge
            url.to_string()
        );

        let hub_token = match command.run() {
            Ok(token) => token,
            Err(_) => panic!()
        };

        let _app_secret_key = 
        sync_engine.register_endpoint(app_name, url.to_string(), hub_token).await;
    }

    println!("Volume mounted");

    let filesystem = FS::new(sync_engine);
    fuse::mount(filesystem, &mountpoint, &options).unwrap();

    Ok(())
}