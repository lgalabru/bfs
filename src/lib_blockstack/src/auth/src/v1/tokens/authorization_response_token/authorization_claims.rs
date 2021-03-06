use crate::v1::tokens::jwt::Salt;
use crate::v1::types::{AuthScope, EncryptedPayload};
use hex;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    // Array of permissions requested by the app
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
    // App private key, encrypted
    #[serde(rename = "privateKey")]
    pub private_key: Option<EncryptedPayload>,
    // App public key
    #[serde(rename = "publicKeys")]
    pub public_keys: Option<Vec<String>>,
}

impl Payload {
    pub fn new(
        address: String,
        association_token: String,
        app_secret_key: EncryptedPayload,
        app_public_keys: Vec<String>,
    ) -> Self {
        // Generate UUID
        let uuid = Uuid::new_v4().to_string();

        // Generate Salt
        let mut rng = thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        let _salt_hex = hex::encode(&salt);

        let did = format!("did:btc-addr:{}", address);

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
            public_keys: Some(app_public_keys),
        }
    }
}
