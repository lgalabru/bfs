use crate::v1::{
    errors::Error,
    helpers::{
        get_master_node_from_bip39_seed,
        get_hardened_derivation
    }
};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256, Digest};
use ripemd160::Ripemd160;
use hex;

pub struct CreateAppKeypair {
    /// User secret seed - BIP39.
    user_bip39_seed: Vec<u8>,
    /// App domain.
    app_domain: String,
    /// Index of the node 'apps'.
    apps_hdn_index: u32,
    /// Index of the identity address to use.
    identity_address_index: u32,
}

impl CreateAppKeypair {

    pub fn new(user_bip39_seed: Vec<u8>, app_domain: String) -> Self {
        Self {
            user_bip39_seed,
            app_domain,
            apps_hdn_index: 0,
            identity_address_index: 0,
        }
    }

    // todo(ludo): comment all the public methods
    pub fn run(&mut self) -> Result<(Vec<u8>, String, String), Error> {

        let (master_node_bytes, chain_code) = get_master_node_from_bip39_seed(&self.user_bip39_seed);
        let master_node = SecretKey::from_slice(&master_node_bytes).unwrap();

        // Derive the idendity node #i: m/888'/0'. 
        // Question: shouldn't we include {identity_address_index}?
        let (identity_node, identity_cc) = get_hardened_derivation(master_node, &chain_code, &[888, 0])?;

        // Compute a salt from this node by hashing its public key
        let salt = {
            let secp = Secp256k1::new();
            let pk = PublicKey::from_secret_key(&secp, &identity_node);
            let public_key = hex::encode(&pk.serialize().to_vec());        
            
            let mut sha2 = Sha256::new();
            sha2.input(public_key.clone());
            let public_key_hashed = sha2.result();
            hex::encode(&public_key_hashed.to_vec())
        };

        // Compute the hash of the app index
        let app_index_hashed = {
            let mut sha2 = Sha256::new();
            sha2.input(format!("{}{}", self.app_domain, salt));
            let public_key_hashed = sha2.result();
            hex::encode(&public_key_hashed.to_vec())
        };

        // Compute the app index. 
        // Tedious: we need to reproduce the underflows / overflows
        // of the javascript's implementation.
        let app_index = {
            let mut index = 0i32;
            for c in app_index_hashed.as_bytes().iter() {
                let s1: i32 = index << 5;
                let s2: i64 = (s1 as i64) - (index as i64) + (*c as i64);
                index = (s2 as i32) & (s2 as i32);
            }
            index & 0x7fffffff
        } as u32;

        // Derive the app node: m/888'/0'/{identity_index}'/{apps_node}'/{app_index}'
        let sub_path = [self.identity_address_index, self.apps_hdn_index, app_index];
        let (app_node, _) = get_hardened_derivation(identity_node, &identity_cc, &sub_path)?;
        
        let public_key = {
            let secp = Secp256k1::new();
            let pk = PublicKey::from_secret_key(&secp, &app_node);
            &pk.serialize().to_vec()
        };

        let address = {
            // SHA256
            let mut sha2 = Sha256::new();
            sha2.input(public_key.clone());
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
            bs58::encode(v_pub_key_h160_checksumed).into_string()
        };

        let sk = hex::decode(&app_node.to_string()).unwrap();
        let pk = hex::encode(&public_key);

        Ok((sk, pk, address.to_string()))
    }
}
