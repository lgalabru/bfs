mod create_app_keypair;
mod create_association_token;
mod create_authorization_token;
mod encrypt_content;
mod verify_authorization_token;

use crate::v1::errors::Error;
use crate::v1::helpers::{
    get_hardened_child_keypair
};
use secp256k1::{Secp256k1, SecretKey};

pub fn get_hardened_m_0(bip39_seed: &str) -> Result<(Vec<u8>, String), Error> {
    get_hardened_child_keypair(bip39_seed, &[0])
}
