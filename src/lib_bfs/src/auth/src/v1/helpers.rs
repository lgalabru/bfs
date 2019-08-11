use crate::v1::errors::Error;
use ring::hmac::{Context, Key, HMAC_SHA512};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex;

pub fn get_hardened_child_keypair(bip39_seed: &str, path: &[u32]) -> Result<(Vec<u8>, String), Error> {
    let (master_node_bytes, chain_code) = get_master_node_from_bip39_seed(&bip39_seed);
    let master_node = SecretKey::from_slice(&master_node_bytes).unwrap();
    let (first_hardened_child, _) = get_hardened_derivation(master_node, &chain_code, &path)?;
    let sk = hex::decode(&first_hardened_child.to_string()).unwrap();
    let pk = {
        let secp = Secp256k1::new();
        let key = PublicKey::from_secret_key(&secp, &first_hardened_child).serialize();
        hex::encode(&key.to_vec())
    };
    Ok((sk, pk))
}

pub fn get_master_node_from_bip39_seed(bip39_seed: &str) -> (Vec<u8>, Vec<u8>) {
    let bytes = hex::decode(&bip39_seed).unwrap();
    let key = Key::new(HMAC_SHA512, b"Bitcoin seed");
    let tag = ring::hmac::sign(&key, &bytes);
    let mut master_node = tag.as_ref().to_vec();
    let chain_code = master_node.split_off(32);
    (master_node, chain_code)
}

pub fn get_hardened_derivation(root_key: SecretKey, root_code: &Vec<u8>, path: &[u32]) -> Result<(SecretKey, Vec<u8>), Error> {
    let mut parent_key = root_key;
    let mut parent_chain_code = root_code.to_vec();

    for index in path.iter() {
        // Hardened keys: [2^31: 2^32)
        let index = 2u32.pow(31) + index;
        // todo(ludo): check index in bound

        // Create signature
        let tag = {
            let key = Key::new(HMAC_SHA512, &parent_chain_code);
            let mut context = Context::with_key(&key);
            context.update(&[0x00]);
            context.update(&parent_key[..]);
            context.update(&index.to_be_bytes());
            context.sign()
        };

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
