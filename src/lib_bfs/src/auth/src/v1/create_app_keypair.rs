use crate::v1::errors::Error;
use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ring::{
    digest,
    hmac::{
        Context, 
        Key, 
        HMAC_SHA512,
        sign
    },
};
use hex;

pub struct CreateAppKeypair {
    /// User secret seed - BIP39.
    user_secret_seed: String,
    /// App public key - compressed.
    app_domain: String,
    /// Index of the node 'apps'.
    apps_hdn_index: u32,
    /// Index of the identity address to use.
    identity_address_index: u32,
}

impl CreateAppKeypair {

    pub fn new(user_secret_seed: String, app_domain: String) -> Self {
        Self {
            user_secret_seed,
            app_domain,
            apps_hdn_index: 0,
            identity_address_index: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {

        let bytes = hex::decode(&self.user_secret_seed).unwrap();
        let key = Key::new(HMAC_SHA512, b"Bitcoin seed");
        let tag = sign(&key, &bytes);
        let mut master_node = tag.as_ref().to_vec();
        let chain_code = master_node.split_off(32);

        // Derive the idendity node #i: m/888'/0'/i'
        let (identity_sk, identity_cc) = self.hardened_derivation(&master_node, &chain_code, &[888, 0, self.identity_address_index])?;
        let secp = Secp256k1::new();
        let pk = PublicKey::from_secret_key(&secp, &identity_sk);
        let public_key = hex::encode(&pk.serialize().to_vec());        

        // // SHA256
        // let mut sha2 = Sha256::new();
        // sha2.input(public_key.clone());
        // let public_key_hashed = sha2.result();
        // let salt = hex::encode(&public_key_hashed.to_vec());
        // println!("pk   {:?}", public_key);
        // println!("salt {:?}", salt);

        // build "app node"
            // build salt:
                // get user public key
                // sha256
                // hex
            // Build "AppsNodeKey";
                // hardened key, identityIndex (0 by default) (identityPrivateKeychain.deriveHardened(identityIndex)),
        
        // Build app index
            // Concatenate "{app_domain}{salt}"
            // sha256
            // hex
            // -> 
            // Get hash code
            // function hashCode(string) {
            //     let hash = 0
            //     if (string.length === 0) return hash
            //     for (let i = 0; i < string.length; i++) {
            //         const character = string.charCodeAt(i)
            //         hash = (hash << 5) - hash + character
            //         hash = hash & hash
            //     }
            //     return hash & 0x7fffffff
            // }
        Ok(())
    }

    pub fn hardened_derivation(&mut self, root_key: &Vec<u8>, root_code: &Vec<u8>, path: &[u32]) -> Result<(SecretKey, Vec<u8>), Error> {

        let mut parent_key = SecretKey::from_slice(&root_key).unwrap();
        let mut parent_chain_code = root_code.to_vec();

        for index in path.iter() {
            // Hardened keys: [2^31: 2^32)
            let index = 2u32.pow(31) + index;
            // todo(ludo): check index in bound

            // Create signature
            let key = Key::new(HMAC_SHA512, &parent_chain_code);
            let mut context = Context::with_key(&key);
            context.update(&[0x00]);
            context.update(&parent_key[..]);
            context.update(&index.to_be_bytes());
            let tag = context.sign();

            // Derive key
            let mut node_key = tag.as_ref().to_vec();
            let chain_code = node_key.split_off(32);
            let mut derived_key = SecretKey::from_slice(&node_key).unwrap();//.map_err(|_| { Error::KeyDerivationFailed });
            derived_key.add_assign(&parent_key[..]).unwrap();//.map_err(|_| { Error::KeyDerivationFailed })?;

            parent_key = derived_key;
            parent_chain_code = chain_code.to_vec();
        }
        Ok((parent_key, parent_chain_code))
    }
}
