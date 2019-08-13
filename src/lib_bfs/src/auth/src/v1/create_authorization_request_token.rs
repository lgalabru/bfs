use crate::v1::{
    jwt:: {
        Header,
        authorization_request_claims::Payload
    },
    types::{AuthScope},
    errors::Error
};
use secp256k1::{
    Secp256k1, 
    SecretKey, 
    PublicKey,
    rand::OsRng,
};
use sha2::{Sha256, Digest};

pub struct CreateAuthorizationRequestToken {
    // todo(ludo): add description
    app_domain: String,
    // todo(ludo): add description
    manifest_uri: String,
    // todo(ludo): add description
    redirect_uri: String,
    // todo(ludo): add description
    version: String,
    // todo(ludo): add description
    do_not_include_profile: String,
    // todo(ludo): add description
    supports_hub_url: String,
    // todo(ludo): add description
    scopes: Vec<AuthScope>,
}

impl CreateAuthorizationRequestToken {

    pub fn new(app_domain: String, 
               manifest_uri: String,
               redirect_uri: String,
               version: String,
               scopes: Option<Vec<AuthScope>>,
               do_not_include_profile: bool,
               supports_hub_url: bool) -> Self {
        Self {
            app_domain,
            manifest_uri,
            redirect_uri,
            version,
            scopes,
            do_not_include_profile,
            supports_hub_url
        }
    }

    pub fn run(&self) -> Result<String, Error> {

        let (transit_sk, transit_pk) = {
            let secp = Secp256k1::new();
            let mut rng = OsRng::new().expect("OsRng");
            secp.generate_keypair(&mut rng)
        };

        // Build payload based on authorization claims
        let payload = {
            let payload = Payload::new(
                self.app_domain,
                self.do_not_include_profile,
                self.manifest_uri,
                self.redirect_uri,
                self.scopes,
                self.supports_hub_url
                self.version,
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
            let key = SecretKey::from_slice(&transit_sk).unwrap();
            let sig = secp.sign(&message, &key);
            let sig_serialized = sig.serialize_compact().to_vec();

            let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
            [signing_input, sig_b64].join(".")
        };

        Ok(authorization_request_token)
    }
}
