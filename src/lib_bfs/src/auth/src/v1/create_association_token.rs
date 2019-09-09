use secp256k1::{Secp256k1, Message, SecretKey};
use sha2::{Sha256, Digest};
use base64;
use hex;
use crate::v1::errors::Error;
use crate::v1::jwt::{
    Header,
    association_claims::Payload
};

pub struct CreateAssociationToken {
    /// User secret key
    user_secret_key: Vec<u8>,
    /// User public key
    user_public_key: String,
    /// App public key - compressed.
    app_public_key: String,
}

impl CreateAssociationToken {

    pub fn new(user_secret_key: Vec<u8>, user_public_key: String, app_public_key: String) -> Self {
        Self {
            user_secret_key,
            user_public_key,
            app_public_key
        }
    }

    pub fn run(&self) -> Result<String, Error> {

        // Build AssociationPayload
        let payload = {
            // todo(ludo): remove this clones
            let payload = Payload::new(self.user_public_key.clone(), 
                                       self.app_public_key.clone());
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

        // Build the association token
        let association_token = {
            let signing_input = format!("{}.{}", header, payload);

            // SHA256
            let mut sha2 = Sha256::new();
            sha2.input(signing_input);
            let signing_input_hashed = sha2.result();

            let secp = Secp256k1::signing_only();
            let sk = SecretKey::from_slice(&self.user_secret_key).expect("32 bytes, within curve order");
            let message = Message::from_slice(&signing_input_hashed).expect("32 bytes");
            let sig = secp.sign(&message, &sk);
            let sig_serialized = sig.serialize_compact().to_vec();

            let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
            format!("v1:{}.{}.{}", header, payload, sig_b64)
        };

        Ok(association_token)
    }
}
