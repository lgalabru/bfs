use crate::v1::{
    jwt:: {
        Header,
        authorization_claims::Payload
    },
    types::{AuthScope},
    helpers::{
        get_hardened_child_keypair,
    },
    errors::Error,
    create_app_keypair::CreateAppKeypair,
    create_association_token::CreateAssociationToken,
    encrypt_content::EncryptContent
};
use secp256k1::{
    Secp256k1, 
    SecretKey, 
    Message,
};
use sha2::{Sha256, Digest};


// todo(ludo): re-export commands in mod.rs

pub struct CreateAuthorizationToken {
    /// User secret seed - BIP39.
    // todo(ludo): we should be using m'/888'/0'/0' (or so) instead
    user_bip39_seed: String,
    /// App domain.
    app_domain: String,
    /// JWT token
    authorization_request_token: String,
    // todo(ludo): add description
    gaia_challenge: String,
    // URL to the write path of the user's Gaia hub
    hub_url: String,
    // todo(ludo): add description
    scopes: Option<Vec<AuthScope>>,
    // Public key used for encrypting the app private key
    transit_public_key: Vec<u8>,
    // Identity to use
    identidy_index: u32
}

impl CreateAuthorizationToken {

    pub fn new(user_bip39_seed: String, 
               app_domain: String, 
               authorization_request_token: String,
               gaia_challenge: String,
               hub_url: String,
               scopes: Option<Vec<AuthScope>>,
               transit_public_key: Vec<u8>,
               identidy_index: u32) -> Self {
        Self {
            user_bip39_seed,
            app_domain,
            authorization_request_token,
            gaia_challenge,
            hub_url,
            scopes,
            transit_public_key,
            identidy_index
        }
    }

    pub fn run(&self) -> Result<String, Error> {

        // Get user keypair
        let (user_secret_key, user_public_key) = {
            let derivation_path = [888, 0, self.identidy_index];
            // todo(ludo): create some constants (888, etc)
            get_hardened_child_keypair(&self.user_bip39_seed, &derivation_path)?
        };

        // Create an app keypair
        let (app_sk, app_pk, app_address) = {
            // todo(ludo): remove clones
            let mut command = CreateAppKeypair::new(self.user_bip39_seed.clone(), 
                                                    self.app_domain.clone());
            command.run()?
        };

        // Create association token
        let association_token = {
            // todo(ludo): remove clones
            let mut command = CreateAssociationToken::new(user_secret_key.clone(),
                                                          user_public_key.clone(),
                                                          app_pk.clone());
            command.run()?
        };

        // Encrypt app private key with transit key
        let encrypted_app_sk = {
            let mut command = EncryptContent::new(self.transit_public_key.clone(),
                                                  app_sk.clone());
            command.run()?
        };

        // Build payload based on authorization claims
        let payload = {
            let payload = Payload::new(
                user_public_key,
                association_token,
                encrypted_app_sk,
                vec![app_pk]
            );
            let w_payload_json = serde_json::to_string(&payload);
            if let Err(_) = w_payload_json {
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
            if let Err(_) = w_header_json {
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
            [signing_input, sig_b64].join(".")
        };

        Ok(authorization_token)
    }
}

