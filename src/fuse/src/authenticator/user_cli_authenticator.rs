use async_trait::async_trait;

use std::io::{self};

use bip39::{Language, Mnemonic};

use blockstack::auth::v1::{
    helpers::get_bip39_seed_from_mnemonic,
    tokens::{CreateAuthorizationRequestToken, CreateAuthorizationToken},
};
use blockstack::storage::{
    AuthenticationDelegate, AuthenticationError, AuthenticationResult, AuthenticationToken,
};

pub struct UserCliAuthenticator;

impl UserCliAuthenticator {
    fn start_authorization_flow(&self) -> AuthenticationResult {
        println!("Mnemonic (12 words):");

        let mut input = String::new();

        let mut phrase = match io::stdin().read_line(&mut input) {
            Ok(_) => input,
            Err(_) => return Err(AuthenticationError::Unknown),
        };
        phrase.pop();
        let _mnemonic = match Mnemonic::from_phrase(phrase.clone(), Language::English) {
            Ok(mnemonic) => mnemonic,
            Err(_) => return Err(AuthenticationError::Unknown),
        };

        let bip39_seed = match get_bip39_seed_from_mnemonic(&phrase, "") {
            Ok(bip39_seed) => bip39_seed,
            Err(_) => return Err(AuthenticationError::Unknown),
        };

        let app_domain = "https://blackhole.run";

        let command = CreateAuthorizationRequestToken::new(
            app_domain.to_string(),
            app_domain.to_string(),
            app_domain.to_string(),
            "1.0".to_string(),
            vec![], // todo(ludo): fill
            true,   // todo(ludo): fill
            true,   // todo(ludo): fill
        );

        let (authorization_request_token, _transit_secret_key) = match command.run() {
            Ok(token) => token,
            Err(_) => return Err(AuthenticationError::Unknown),
        };

        let command = CreateAuthorizationToken::new(
            bip39_seed,
            authorization_request_token,
            "".to_string(), // todo(ludo): fill gaia_challenge
            "".to_string(), // todo(ludo): fill hub_url
            0,
        );

        let authorization_token = match command.run() {
            Ok(token) => token,
            Err(_) => return Err(AuthenticationError::Unknown),
        };

        Ok(AuthenticationToken::new(&authorization_token))
    }
}

#[async_trait]
impl AuthenticationDelegate for UserCliAuthenticator {
    async fn get_authorization_token(&self) -> AuthenticationResult {
        // Start authorization flow
        let result = self.start_authorization_flow();
        if let Err(err) = result {
            return Err(err);
        }
        // Generate all the keypairs

        let token = result.unwrap();
        println!("Authorization succeeded! {:?}", token);
        Ok(token)
    }
}
