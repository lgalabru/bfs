use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use rand::{Rng, thread_rng};
use base64;
use hex;
use crate::v1::errors::Error;
use crate::v1::jwt;

pub struct CreateAssociationToken {
    /// User private key - WIF.
    user_secret_key: String,
    /// App public key - compressed.
    app_public_key: String,
}

impl CreateAssociationToken {

    pub fn new(user_secret_key: String, app_public_key: String) -> Self {
        Self {
            user_secret_key,
            app_public_key
        }
    }

    pub fn run(&self) -> Result<(), Error> {
        // Create lifetime

        // Compute public key
        let (user_public_key, user_secret_key) = self.get_user_key_pair()?;

        // Generate Salt
        let mut rng = thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        let salt_hex = hex::encode(&salt);

        // Build AssociationPayload:
        let payload = jwt::association_claims::Payload::new(user_public_key, self.app_public_key.clone(), salt_hex);
        let w_payload_json = serde_json::to_string(&payload);
        if let Err(_) = w_payload_json {
            // Unable to serialize JWT's payload
            return Err(Error::PayloadDataCorrupted);
        }
        let payload_json = w_payload_json.unwrap();
        let payload_b64 = base64::encode_config(&payload_json, base64::URL_SAFE_NO_PAD);

        // Build header
        let header = jwt::Header::new();
        let w_header_json = serde_json::to_string(&header);
        if let Err(_) = w_header_json {
            // Unable to serialize JWT's header
            return Err(Error::HeaderDataCorrupted);
        }
        let header_json = w_header_json.unwrap();
        let header_b64 = base64::encode_config(&header_json, base64::URL_SAFE_NO_PAD);

        let signing_input = [header_b64, payload_b64].join(".");

        // SHA256
        let mut sha2 = Sha256::new();
        sha2.input(signing_input.clone());
        let signing_input_hashed = sha2.result();

        let secp = Secp256k1::new();
        let message = Message::from_slice(&signing_input_hashed).expect("32 bytes");
        let sig = secp.sign(&message, &user_secret_key);
        let sig_serialized = sig.serialize_compact().to_vec();

        let sig_b64 = base64::encode_config(&sig_serialized, base64::URL_SAFE);
        let association_token = [signing_input, sig_b64].join(".");

        println!("{:?}", association_token);

        Ok(())
    }

    fn decoded_user_secret_key(&self) -> Result<Vec<u8>, Error> {
        // Decode base58
        let sk_checksumed = bs58::decode(&self.user_secret_key).into_vec().unwrap();
        let len = sk_checksumed.len();
        let suffix_len: usize;
        let mut should_compress = false;

        if len == 32 + 4 + 1 + 1 {
            suffix_len = 5;
            should_compress = true;
        } else if len == 32 + 4 + 1 {
            suffix_len = 4;
        } else {
            return Err(Error::SecretKeyCorrupted);
        }
        let sk = &sk_checksumed[1..len-suffix_len];

        // Should handle checksum
        // Should handle 0x80

        Ok(sk.to_vec())
    }

    pub fn get_user_key_pair(&self) -> Result<(String, secp256k1::key::SecretKey), Error> {
        let secret_key = self.decoded_user_secret_key()?;

        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&secret_key).expect("32 bytes, within curve order");
        let pk = PublicKey::from_secret_key(&secp, &sk);
        let public_key = pk.serialize();
        let public_key_hex = hex::encode(&public_key.to_vec());

        Ok((public_key_hex, sk))
    }
}
