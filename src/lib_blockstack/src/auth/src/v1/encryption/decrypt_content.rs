use crate::v1::{
    types::{EncryptedPayload},
    errors::Error,
};
use secp256k1::{
    SecretKey, 
    PublicKey,
    ecdh::SharedSecret
};
// use ring::hmac::{Context, Key, HMAC_SHA256};
use sha2::{Sha512, Digest};
use block_modes::{
    block_padding::Pkcs7,
    BlockMode, 
    Cbc
};
use aes::Aes256;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct DecryptContent {
    /// Secret key used for decrypting the data
    secret_key: Vec<u8>,
    /// Data to encrypt.
    encrypted_payload: EncryptedPayload,
}

impl DecryptContent {

    pub fn new(secret_key: Vec<u8>, encrypted_payload: EncryptedPayload) -> Self {
        Self {
            secret_key,
            encrypted_payload
        }
    }

    pub fn run(&self) -> Result<Vec<u8>, Error> {

        // Extract payload
        // todo(ludo): handle all the unwrap()
        let ephemeral_pk = {
            let data = self.encrypted_payload.ephemeral_pk.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let iv = {
            let data = self.encrypted_payload.iv.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let _signature = {
            let data = self.encrypted_payload.mac.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let cipher_text = {
            let data = self.encrypted_payload.cipher_text.as_ref().unwrap();
            hex::decode(data).unwrap()
        };

        // Generate a shared secret
        let mut shared_secret = {
            let pk = PublicKey::from_slice(&ephemeral_pk).unwrap();
            let sk = SecretKey::from_slice(&self.secret_key).unwrap();
            let shared_secret = SharedSecret::new(&pk, &sk);
                        
            let mut hasher = Sha512::new();
            hasher.input(&shared_secret[..]);
            hasher.result().to_vec()
        };


        // todo(ludo): check hmac
        let _hmac_key = shared_secret.split_off(32);
        // let tag = {
        //     let key = Key::new(HMAC_SHA256, &hmac_key);
        //     let mut context = Context::with_key(&key);
        //     context.update(&iv[..]);
        //     context.update(&ephemeral_pk.as_bytes());
        //     context.update(&ephemeral_pk.as_bytes());
        //     context.update(&cipher_text);
        //     context.sign()
        // };

        let cipher = Aes256Cbc::new_var(&shared_secret, &iv).unwrap();
        let mut buf = cipher_text.to_vec();
        let data = cipher.decrypt(&mut buf).unwrap();

        Ok(data.to_vec())
    }
}
