use crate::v1::errors::Error;
use ring::hmac::{Context, Key, HMAC_SHA512};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use hex;

pub fn get_hardened_child_keypair(bip39_seed: &Vec<u8>, path: &[u32]) -> Result<(Vec<u8>, String), Error> {
    let (master_node_bytes, chain_code) = get_master_node_from_bip39_seed(&bip39_seed);
    let master_node = SecretKey::from_slice(&master_node_bytes).unwrap();
    let (sk, _) = get_hardened_derivation(master_node, &chain_code, &path)?;
    let secp = Secp256k1::new();
    let pk = PublicKey::from_secret_key(&secp, &sk);
    export_keypair(sk, pk)
}

// todo(ludo): Revisit this strategy. Should intensively use pk[..] instead.
pub fn export_keypair(secret_key: SecretKey, public_key: PublicKey) -> Result<(Vec<u8>, String), Error> {
    let sk = hex::decode(&secret_key.to_string()).unwrap();
    let pk = hex::encode(&public_key.serialize().to_vec());
    Ok((sk, pk))
}

pub fn get_master_node_from_bip39_seed(bip39_seed: &Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    let key = Key::new(HMAC_SHA512, b"Bitcoin seed");
    let tag = ring::hmac::sign(&key, &bip39_seed);
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

pub fn get_private_key_from_wif(wif: &str) -> Result<Vec<u8>, Error> {
    // Decode base58
    let sk_checksumed = bs58::decode(&wif).into_vec().unwrap();
    let len = sk_checksumed.len();
    let suffix_len: usize;
    let mut should_compress = false;

    // todo(ludo): Improve legibility
    if len == 32 + 4 + 1 + 1 {
        suffix_len = 5;
        should_compress = true;
    } else if len == 32 + 4 + 1 {
        suffix_len = 4;
    } else {
        return Err(Error::SecretKeyCorrupted);
    }
    let sk = &sk_checksumed[1..len-suffix_len];

    // todo(ludo): Should handle checksum
    // todo(ludo): Should handle 0x80

    Ok(sk.to_vec())
}

pub fn get_address_from_public_key(public_key: &str) -> Result<String, Error> {
    let pub_key_hex = hex::decode(&public_key).unwrap();

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
    Ok(bs58::encode(v_pub_key_h160_checksumed).into_string())
}
