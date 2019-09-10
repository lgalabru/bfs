use crate::v1::{
    tokens::jwt:: {
        Header
    },
    types::{AuthScope},
    errors::Error,
    helpers::get_address_from_public_key
};
use super::authorization_request_claims::Payload;
use secp256k1::{
    Secp256k1, 
    Message,
    rand::OsRng,
};
use sha2::{Sha256, Digest};

pub struct CreateAuthorizationRequestToken {
    // Blockstack apps are uniquely identified by their app domain
    app_domain: String,
    // URI of app's manifest file
    manifest_uri: String,
    // URI to redirect users to after authentication
    redirect_uri: String,
    // todo(ludo): add description
    version: String,
    // todo(ludo): add description
    do_not_include_profile: bool,
    // todo(ludo): add description
    supports_hub_url: bool,
    // Array of permissions requested by the app
    scopes: Vec<AuthScope>,
}

impl CreateAuthorizationRequestToken {

    pub fn new(app_domain: String, 
               manifest_uri: String,
               redirect_uri: String,
               version: String,
               scopes: Vec<AuthScope>,
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

    pub fn run(&self) -> Result<(String, Vec<u8>), Error> {

        let (transit_sk, transit_pk) = {
            let secp = Secp256k1::new();
            let mut rng = OsRng::new().expect("OsRng");
            secp.generate_keypair(&mut rng)
        };

        let transit_pk_hex = hex::encode(&transit_pk.serialize().to_vec());

        let payload = {
            let address = match get_address_from_public_key(&transit_pk_hex) {
                Ok(address) => address,
                Err(_) => return Err(Error::PayloadDataCorrupted) // todo(ludo): add error
            };

            let payload = Payload::new(
                address,
                self.app_domain.clone(),
                self.do_not_include_profile,
                self.manifest_uri.clone(),
                self.redirect_uri.clone(),
                self.version.clone(),
                self.scopes.clone(),
                self.supports_hub_url,
                vec![transit_pk_hex]
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

        // Build JWT authorization_request_token
        let authorization_request_token = {
            let signing_input = format!("{}.{}", header, payload);

            let mut sha2 = Sha256::new();
            sha2.input(signing_input);
            let signing_input_hashed = sha2.result();

            let secp = Secp256k1::signing_only();
            let message = Message::from_slice(&signing_input_hashed).expect("32 bytes");
            let sig = secp.sign(&message, &transit_sk);
            let sig_serialized = sig.serialize_compact().to_vec();

            let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
            format!("v1:{}.{}.{}", header, payload, sig_b64)
        };
        let transit_sk_hex = hex::decode(&transit_sk.to_string()).unwrap();

        Ok((authorization_request_token, transit_sk_hex))
    }
}