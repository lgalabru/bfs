use serde_json;
use sha2::{Sha256, Digest};
use ripemd160::Ripemd160;
use bs58;
use base64;

use crate::utils;
use crate::jwt;

// pub struct Error;

/// Hex deserialization error
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Error {
    /// Authorization header should start with 'v1:'
    VersionMismatch,
    /// Tokens should have 3 components
    MalFormattedToken,
    HeaderEncodingCorrupted,
    HeaderDataCorrupted,
    PayloadEncodingCorrupted,
    PayloadDataCorrupted,
    PrincipalMissing,
}

pub struct Authentication {
    /// JWT Token - utf-8 encoded
    token: String
}

impl Authentication {

    pub fn new(token: String) -> Authentication {
        Authentication {
            token
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
        let payload: jwt::Payload = w_payload.unwrap();

        if let None = payload.iss {
            // Auth token should be a JWT with at least an `iss` claim
            return Err(Error::PrincipalMissing);
        }

        let pub_key = payload.iss.unwrap();

        // Get bytes
        let pub_key_hex = utils::hex_bytes(&pub_key).unwrap();

        // SHA256
        let mut sha2 = Sha256::new();
        sha2.input(pub_key_hex);
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
        
        println!("2) - {:?}", address);

        // Todo: Check payload.iss / address against issuerAddress
        // Todo: Check payload.iat against options.oldestValidTokenTimestamp (1)
        // Todo: Check payload.hubUrl against options.validHubUrls
        // Todo: Check payload.scopes
        // Todo: Check payload.gaiaChallenge against challengeTexts
        // Todo: Check payload.exp against time.now
        // Todo: Check payload.associationToken

        // Fun stuff: token verification

        Ok(())
    }
}
