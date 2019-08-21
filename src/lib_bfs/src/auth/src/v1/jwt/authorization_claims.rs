use serde::{Deserialize, Serialize};
use rand::{Rng, thread_rng};
use hex;
use uuid::Uuid;
use crate::v1::types::{AuthScope, EncryptedPayload};
use crate::v1::jwt::{Salt};

/// JWT Claims Set for "Authorization tokens"
#[derive(Serialize, Deserialize)]
pub struct Payload {
    /// Principal that issued the JWT.
    pub iss: Option<String>,
    /// Unique identifier for the JWT.
    pub jti: Option<String>,
    /// Time at which the JWT was issued.
    pub iat: Option<u64>,
    /// Expiration time on or after which the JWT MUST NOT be accepted for processing.
    pub exp: Option<u64>,
    // todo(ludo): add description
    pub scopes: Option<Vec<AuthScope>>,
    // todo(ludo): add description
    #[serde(rename = "gaiaChallenge")]
    pub gaia_challenge: Option<String>,
    /// Add some salt to limit replay attacks
    pub salt: Option<Salt>,
    // todo(ludo): add description
    #[serde(rename = "hubUrl")]
    pub hub_url: Option<String>,
    // todo(ludo): add description
    #[serde(rename = "associationToken")]
    pub association_token: Option<String>,
    // Private key, encrypted
    // todo(ludo): add description
    #[serde(rename = "privateKey")]
    pub private_key: Option<EncryptedPayload>,
    // todo(ludo): add description
    #[serde(rename = "publicKeys")]
    pub public_keys: Option<Vec<String>>,
}

impl Payload {

    pub fn new(address: String, 
               association_token: String, 
               app_secret_key: EncryptedPayload,
               app_public_keys: Vec<String>) -> Self {

        // Generate UUID
        let uuid = Uuid::new_v4().to_string();

        // Generate Salt
        let mut rng = thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        let salt_hex = hex::encode(&salt);

        let did = format!("did:btc-addr:{}", address);

        // todo(ludo): encode secret key

        Self {
            jti: Some(uuid),
            // todo(ludo): set issued at
            iat: Some(0),
            // todo(ludo): set expiration
            exp: Some(0),
            iss: Some(did),
            // todo(ludo): set the salt correctly - Some(salt_hex),
            salt: None,
            gaia_challenge: None,
            scopes: None,
            hub_url: None,
            association_token: Some(association_token),
            private_key: Some(app_secret_key),
            public_keys: Some(app_public_keys)
        }
    }
}
