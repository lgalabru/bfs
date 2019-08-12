use crate::v1::{
    types::{EncryptedPayload},
    helpers::{
        get_hardened_child_keypair,
        export_keypair
    },
    errors::Error,
};
use secp256k1::{
    Secp256k1, 
    SecretKey, 
    PublicKey,
    ecdh::SharedSecret
};
use ring::hmac::{Context, Key, HMAC_SHA256};
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

        // todo(ludo): handle all the unwrap()
        let ephemeral_pk = {
            let data = self.encrypted_payload.ephemeral_pk.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let iv = {
            let data = self.encrypted_payload.iv.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let signature = {
            let data = self.encrypted_payload.mac.as_ref().unwrap();
            hex::decode(data).unwrap()
        };
        let cipher_text = {
            let data = self.encrypted_payload.cipher_text.as_ref().unwrap();
            hex::decode(data).unwrap()
        };

        let mut shared_secret = {
            let pk = PublicKey::from_slice(&ephemeral_pk).unwrap();
            let sk = SecretKey::from_slice(&self.secret_key).unwrap();
            let mut shared_secret = SharedSecret::new(&pk, &sk);
            
            let secp = Secp256k1::new();
            let pk2 = PublicKey::from_secret_key(&secp, &sk);

            &shared_secret[..].to_vec()
        };

        // todo(ludo): sha512 of shared_secret

        let mut shared_secret = shared_secret.clone();

        let hmac_key = shared_secret.split_off(32);

        // todo(ludo): check hmac

        let cipher = Aes256Cbc::new_var(&shared_secret, &iv).unwrap();
        let mut buf = cipher_text.to_vec();
        let data = cipher.decrypt(&mut buf).unwrap();
        println!("{:?}", data);

        // let tag = {
        //     let key = Key::new(HMAC_SHA256, &hmac_key);
        //     let mut context = Context::with_key(&key);
        //     context.update(&iv[..]);
        //     context.update(&ephemeral_pk.as_bytes());
        //     context.update(&ephemeral_pk.as_bytes());
        //     context.update(&cipher_text);
        //     context.sign()
        // };

        Ok(data.to_vec())
    }
}
