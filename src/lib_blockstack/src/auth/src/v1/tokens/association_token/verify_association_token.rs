use base64;
use hex;
use secp256k1::{Message, PublicKey, Secp256k1, Signature};
use serde_json;
use sha2::{Digest, Sha256};

use super::association_claims::Payload;
use crate::v1::{errors::Error, tokens::jwt::Header};

pub struct VerifyAssociationToken {
    // todo(ludo): add description
    token: String,
}

impl VerifyAssociationToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn run(&mut self) -> Result<(Header, Payload), Error> {
        let version_prefix = "v1:";

        let (version, jwt_token) = self.token.split_at(version_prefix.len());
        if version != version_prefix {
            return Err(Error::VersionMismatch);
        }

        let jwt_parts: Vec<&str> = jwt_token.split('.').collect();

        if jwt_parts.len() != 3 {
            // Tokens should have 3 components
            return Err(Error::MalFormattedToken);
        }

        let signature = jwt_parts[2];

        let header: Header = {
            let w_header_decoded = base64::decode(jwt_parts[0]);
            if w_header_decoded.is_err() {
                // Unable to base64 decode JWT's header
                return Err(Error::HeaderEncodingCorrupted);
            }
            let header_decoded = w_header_decoded.unwrap();

            let w_header = serde_json::from_slice(&header_decoded[..]);
            if w_header.is_err() {
                // Unable to deserialize JWT's header
                return Err(Error::HeaderDataCorrupted);
            }
            w_header.unwrap()
        };

        let payload: Payload = {
            let w_payload_decoded = base64::decode(jwt_parts[1]);
            if w_payload_decoded.is_err() {
                // Unable to base64 decode JWT's payload
                return Err(Error::PayloadEncodingCorrupted);
            }
            let payload_decoded = w_payload_decoded.unwrap();

            let w_payload = serde_json::from_slice(&payload_decoded[..]);
            if w_payload.is_err() {
                // Unable to deserialize JWT's payload
                return Err(Error::PayloadDataCorrupted);
            }
            w_payload.unwrap()
        };

        let public_key = match payload.iss {
            Some(ref public_key) => hex::decode(&public_key).unwrap(),
            None => return Err(Error::PayloadDataCorrupted), // todo(ludo): custom error
        };

        // Check Signature
        let sig_verification = {
            let w_url_safe_b64_decode = base64::decode_config(&signature, base64::URL_SAFE);
            if w_url_safe_b64_decode.is_err() {
                // Unable to base64 decode JWT's payload
                return Err(Error::SignatureEncodingCorrupted);
            }
            let compact_sig = w_url_safe_b64_decode.unwrap();

            let signing_input = format!("{}.{}", jwt_parts[0], jwt_parts[1]);

            // SHA256
            let mut sha2 = Sha256::new();
            sha2.input(signing_input.clone());
            let signing_input_hashed = sha2.result();

            // Verify signature
            let secp = Secp256k1::verification_only();

            let public_key = PublicKey::from_slice(&public_key)
                .expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
            let message = Message::from_slice(&signing_input_hashed)
                .expect("messages must be 32 bytes and are expected to be hashes");
            let sig =
                Signature::from_compact(&compact_sig).expect("compact signatures are 64 bytes;");
            secp.verify(&message, &sig, &public_key)
        };

        assert!(sig_verification.is_ok());

        // todo(ludo): Check payload.iat against options.oldestValidTokenTimestamp (1)
        // todo(ludo): Check payload.exp against time.now

        Ok((header, payload))
    }
}
