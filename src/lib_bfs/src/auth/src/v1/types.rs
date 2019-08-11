use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthScope {
    pub scope: Option<String>,
    pub domain: Option<String>,
}

// export const AuthScopes = [
//   'putFile',
//   'putFilePrefix',
//   'deleteFile',
//   'deleteFilePrefix'
// ]

#[derive(Serialize, Deserialize)]
pub struct EncryptedPayload {
    // todo(ludo): add description
    pub iv: Option<String>,
    // todo(ludo): add description, serialize to ephemeralPK
    pub ephemeral_pk: Option<String>,
    // todo(ludo): add description, serialize to cipherText
    pub cipher_text: Option<String>,
    // todo(ludo): add description
    pub mac: Option<String>,
    // todo(ludo): add description, move to bool?, serialize to wasString
    pub was_string: Option<String>,
}

impl EncryptedPayload {
    pub fn new(iv: String, ephemeral_pk: String, cipher_text: String, mac: String, was_string: String) -> Self {
        Self {
            iv: Some(iv),
            ephemeral_pk: Some(ephemeral_pk),
            cipher_text: Some(cipher_text),
            mac: Some(mac),
            was_string: Some(was_string)
        }
    }
}