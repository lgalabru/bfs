use super::hub_claims::Payload;
use crate::v1::errors::Error;
use crate::v1::tokens::jwt::Header;
use base64;
use hex;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub struct CreateHubToken {
    /// App secret key
    app_secret_key: Vec<u8>,
    // todo(ludo): add description
    hub_url: String,
    // todo(ludo): add description
    gaia_challenge: String,
}

impl CreateHubToken {
    pub fn new(app_secret_key: Vec<u8>, gaia_challenge: String, hub_url: String) -> Self {
        Self {
            app_secret_key,
            hub_url,
            gaia_challenge,
        }
    }

    pub fn run(&self) -> Result<String, Error> {
        let secp = Secp256k1::new();
        let secret_key =
            SecretKey::from_slice(&self.app_secret_key).expect("32 bytes, within curve order");;
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let public_key_hex = hex::encode(&public_key.serialize().to_vec());

        // Build AssociationPayload
        let payload = {
            // todo(ludo): remove this clones
            let payload = Payload::new(
                public_key_hex,
                self.hub_url.clone(),
                self.gaia_challenge.clone(),
            );
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

        // Build the association token
        let hub_token = {
            // todo(ludo): merge slices instead
            let signing_input = [header, payload].join(".");

            // SHA256
            let mut sha2 = Sha256::new();
            sha2.input(signing_input.clone());
            let signing_input_hashed = sha2.result();

            let secp = Secp256k1::signing_only();
            let message = Message::from_slice(&signing_input_hashed).expect("32 bytes");
            let sig = secp.sign(&message, &secret_key);
            let sig_serialized = sig.serialize_compact().to_vec();

            let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
            format!("v1:{}.{}", signing_input, sig_b64)
        };

        Ok(hub_token)
    }
}
