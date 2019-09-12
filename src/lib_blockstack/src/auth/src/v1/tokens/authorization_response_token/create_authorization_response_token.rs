use super::authorization_claims::Payload;
use crate::v1::{
    // Commands
    encryption::EncryptContent,
    errors::Error,
    helpers::{get_address_from_public_key, get_hardened_child_keypair},
    tokens::{
        jwt::Header, CreateAppKeypair, CreateAssociationToken, VerifyAuthorizationRequestToken,
    },
};

use secp256k1::{Message, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

// todo(ludo): re-export commands in mod.rs

pub struct CreateAuthorizationToken {
    /// User secret seed - BIP39.
    // todo(ludo): we should be using m'/888'/0'/0' (or so) instead
    user_bip39_seed: Vec<u8>,
    /// JWT token
    authorization_request_token: String,
    // todo(ludo): add description
    gaia_challenge: String,
    // URL to the write path of the user's Gaia hub
    hub_url: String,
    // Identity to use
    identidy_index: u32,
}

impl CreateAuthorizationToken {
    pub fn new(
        user_bip39_seed: Vec<u8>,
        authorization_request_token: String,
        gaia_challenge: String,
        hub_url: String,
        identidy_index: u32,
    ) -> Self {
        Self {
            user_bip39_seed,
            authorization_request_token,
            gaia_challenge,
            hub_url,
            identidy_index,
        }
    }

    pub fn run(&self) -> Result<String, Error> {
        // Verify + Extract authorization request token
        let mut command =
            VerifyAuthorizationRequestToken::new(self.authorization_request_token.clone());
        let (_, auth_request_payload) = match command.run() {
            Ok(res) => res,
            Err(_) => return Err(Error::PayloadDataCorrupted), // todo(ludo): Add specific error
        };

        let public_transit_key = match auth_request_payload.public_keys {
            Some(public_keys) => hex::decode(&public_keys[0]).unwrap(),
            None => return Err(Error::PayloadDataCorrupted), // todo(ludo): Add specific error
        };

        let app_domain = match auth_request_payload.app_domain {
            Some(app_domain) => app_domain,
            None => return Err(Error::PayloadDataCorrupted), // todo(ludo): Add specific error
        };

        // Get user keypair
        let (user_secret_key, user_public_key) = {
            let derivation_path = [888, 0, self.identidy_index];
            // todo(ludo): create some constants (888, etc)
            get_hardened_child_keypair(&self.user_bip39_seed, &derivation_path)?
        };

        // Create an app keypair
        let (app_sk, app_pk, _app_address) = {
            // todo(ludo): remove clones
            let mut command =
                CreateAppKeypair::new(self.user_bip39_seed.clone(), app_domain.clone());
            command.run()?
        };

        // Create association token
        let association_token = {
            // todo(ludo): remove clones
            let command = CreateAssociationToken::new(
                user_secret_key.clone(),
                user_public_key.clone(),
                app_pk.clone(),
            );
            command.run()?
        };

        // Encrypt app private key with transit key
        let encrypted_app_sk = {
            let command = EncryptContent::new(public_transit_key.clone(), app_sk.clone());
            command.run()?
        };

        // Build payload based on authorization claims
        let payload = {
            let address = get_address_from_public_key(&user_public_key).unwrap(); // todo(ludo): handle unwrap()
            let payload = Payload::new(address, association_token, encrypted_app_sk, vec![app_pk]);
            let w_payload_json = serde_json::to_string(&payload);
            if w_payload_json.is_err() {
                // Unable to serialize JWT's payload
                return Err(Error::PayloadDataCorrupted);
            }
            let payload_json = w_payload_json.unwrap();
            base64::encode_config(&payload_json, base64::URL_SAFE_NO_PAD)
        };

        // Build header
        let header = {
            let header = Header::new();
            let w_header_json = serde_json::to_string(&header);
            if w_header_json.is_err() {
                // Unable to serialize JWT's header
                return Err(Error::HeaderDataCorrupted);
            }
            let header_json = w_header_json.unwrap();
            base64::encode_config(&header_json, base64::URL_SAFE_NO_PAD)
        };

        let authorization_token = {
            // todo(ludo): merge slices instead
            let signing_input = [header, payload].join(".");

            // SHA256
            let mut sha2 = Sha256::new();
            sha2.input(signing_input.clone());
            let signing_input_hashed = sha2.result();

            let secp = Secp256k1::signing_only();
            let message = Message::from_slice(&signing_input_hashed).expect("32 bytes");
            let key = SecretKey::from_slice(&user_secret_key).unwrap();
            let sig = secp.sign(&message, &key);
            let sig_serialized = sig.serialize_compact().to_vec();

            let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
            format!("v1:{}.{}", signing_input, sig_b64)
        };

        Ok(authorization_token)
    }
}
