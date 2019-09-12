use crate::v1::{errors::Error, types::EncryptedPayload};
use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{thread_rng, Rng};
use ring::hmac::{Context, Key, HMAC_SHA256};
use secp256k1::rand::OsRng;
use secp256k1::{ecdh::SharedSecret, PublicKey, Secp256k1};
use sha2::{Digest, Sha512};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct EncryptContent {
    /// Public key used for encrypting the data
    public_key: Vec<u8>,
    /// Data to encrypt.
    data: Vec<u8>,
}

impl EncryptContent {
    pub fn new(public_key: Vec<u8>, data: Vec<u8>) -> Self {
        Self { public_key, data }
    }

    pub fn run(&self) -> Result<EncryptedPayload, Error> {
        // Generate the initialization vector
        let mut rng = thread_rng();
        let mut iv = [0u8; 16];
        rng.fill(&mut iv);

        // Generate an ephemeral keypair
        let (mut shared_secret, ephemeral_pk) = {
            let secp = Secp256k1::new();
            let mut rng = OsRng::new().expect("OsRng");
            let (ephemeral_sk, ephemeral_pk) = secp.generate_keypair(&mut rng);
            let pk = PublicKey::from_slice(&self.public_key).unwrap();
            // todo(ludo): is this ECDH SharedSecret compatible with the JS implementation?
            let shared_secret = SharedSecret::new(&pk, &ephemeral_sk);
            let mut hasher = Sha512::new();
            hasher.input(&shared_secret[..]);
            (
                hasher.result().to_vec(),
                hex::encode(&ephemeral_pk.serialize().to_vec()),
            )
        };

        let hmac_key = shared_secret.split_off(32);

        let cipher = Aes256Cbc::new_var(&shared_secret, &iv).unwrap();

        // todo(ludo): improve buffer size handling (must have enough space for message+padding)
        let mut buffer = [0u8; 64];
        let pos = self.data.len();
        buffer[..pos].copy_from_slice(&self.data);
        let cipher_text = cipher.encrypt(&mut buffer, pos).unwrap();

        // Create signature
        let tag = {
            let key = Key::new(HMAC_SHA256, &hmac_key);
            let mut context = Context::with_key(&key);
            context.update(&iv[..]);
            context.update(&ephemeral_pk.as_bytes());
            context.update(&cipher_text);
            context.sign()
        };

        let was_string = "".to_string();

        Ok(EncryptedPayload::new(
            hex::encode(&iv),
            ephemeral_pk,
            hex::encode(&cipher_text),
            hex::encode(tag.as_ref()),
            was_string,
        ))
    }
}
