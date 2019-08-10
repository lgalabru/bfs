use serde_json;
use sha2::{Sha256, Digest};
use ripemd160::Ripemd160;
use bs58;
use base64;
use secp256k1::{Secp256k1, Message, Signature, PublicKey};

use crate::utils;
use crate::v1::jwt;
use crate::v1::errors::Error;

pub struct VerifyAuthorizationToken {
    token: String,
    issuer_address: Option<String>,
    valid_hub_urls: Option<Vec<String>>,
    challenge_texts: Option<Vec<String>>,
}

impl VerifyAuthorizationToken {

    pub fn new(token: String) -> Self {
        Self {
            token,
            issuer_address: None,
            valid_hub_urls: None,
            challenge_texts: None
        }
    }

    pub fn verify() -> Result<(), Error> {

        Ok(())
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        let version_prefix = "v1:";
        let (version, jwt_token) = self.token.split_at(version_prefix.len());
        if version != version_prefix {
            return Err(Error::VersionMismatch);
        }

        let jwt_parts: Vec<&str> = jwt_token.split(".").collect();

        if jwt_parts.len() != 3 {
            // Tokens should have 3 components
            return Err(Error::MalFormattedToken);
        }

        let signing_input = [jwt_parts[0].clone(), jwt_parts[1].clone()].join(".");
        let signature = jwt_parts[2];

        let w_header_decoded = base64::decode(jwt_parts[0]);
        if let Err(_) = w_header_decoded {
            // Unable to base64 decode JWT's header
            return Err(Error::HeaderEncodingCorrupted);
        }
        let header_decoded = w_header_decoded.unwrap();

        let w_header = serde_json::from_slice(&header_decoded[..]);
        if let Err(_) = w_header {
            // Unable to deserialize JWT's header
            return Err(Error::HeaderDataCorrupted);
        }
        let header: jwt::Header = w_header.unwrap();

        let w_payload_decoded = base64::decode(jwt_parts[1]);
        if let Err(_) = w_payload_decoded {
            // Unable to base64 decode JWT's payload
            return Err(Error::PayloadEncodingCorrupted);
        }
        let payload_decoded = w_payload_decoded.unwrap();

        let w_payload = serde_json::from_slice(&payload_decoded[..]);
        if let Err(_) = w_payload {
            // Unable to deserialize JWT's payload
            return Err(Error::PayloadDataCorrupted);
        }
        let payload: jwt::authorization_claims::Payload = w_payload.unwrap();

        if let None = payload.iss {
            // Auth token should be a JWT with at least an `iss` claim
            return Err(Error::PrincipalMissing);
        }

        let pub_key = payload.iss.unwrap();

        // Get bytes
        let pub_key_hex = utils::hex_bytes(&pub_key).unwrap();

        // SHA256
        let mut sha2 = Sha256::new();
        sha2.input(pub_key_hex.clone());
        let pub_key_hashed = sha2.result();

        // RIPEMD160
        let mut rmd = Ripemd160::new();
        let mut pub_key_h160 = [0u8; 20];
        rmd.input(pub_key_hashed);
        pub_key_h160.copy_from_slice(rmd.result().as_slice());

        // Prepend version byte
        let version_byte = [0]; // MAINNET_SINGLESIG
        let v_pub_key_h160 = [&version_byte[..], &pub_key_h160[..]].concat();
        
        // Append checksum
        let mut sha2_1 = Sha256::new();
        sha2_1.input(v_pub_key_h160.clone());
        let mut sha2_2 = Sha256::new();
        sha2_2.input(sha2_1.result().as_slice());
        let checksum = sha2_2.result();
        let v_pub_key_h160_checksumed = [&v_pub_key_h160[..], &checksum[0..4]].concat();
        
        // Base58 encode
        let address = bs58::encode(v_pub_key_h160_checksumed).into_string();

        // Check Signature
        let w_url_safe_b64_decode = base64::decode_config(&signature, base64::URL_SAFE);
        if let Err(_) = w_url_safe_b64_decode {
            // Unable to base64 decode JWT's payload
            return Err(Error::SignatureEncodingCorrupted);
        }
        let compact_sig = w_url_safe_b64_decode.unwrap();

        let signing_input = [jwt_parts[0].clone(), jwt_parts[1].clone()].join(".");
        
        // SHA256
        let mut sha2 = Sha256::new();
        sha2.input(signing_input.clone());
        let signing_input_hashed = sha2.result();

        let secp = Secp256k1::verification_only();
        let public_key = PublicKey::from_slice(&pub_key_hex).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
        let message = Message::from_slice(&signing_input_hashed).expect("messages must be 32 bytes and are expected to be hashes");
        let sig = Signature::from_compact(&compact_sig).expect("compact signatures are 64 bytes;");
        assert!(secp.verify(&message, &sig, &public_key).is_ok());

        // Todo: Check payload.iss / address against issuerAddress
        // Todo: Check payload.iat against options.oldestValidTokenTimestamp (1)
        // Todo: Check payload.hubUrl against options.validHubUrls
        // Todo: Check payload.scopes
        // Todo: Check payload.gaiaChallenge against challengeTexts
        // Todo: Check payload.exp against time.now
        // Todo: Check payload.associationToken

        Ok(())
    }
}
