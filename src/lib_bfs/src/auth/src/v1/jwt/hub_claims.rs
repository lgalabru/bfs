use serde::{Deserialize, Serialize};
use rand::{Rng, thread_rng};
use hex;
use uuid::Uuid;
use crate::v1::jwt::{Salt};

/// JWT Claims Set for "Association tokens"
#[derive(Serialize, Deserialize)]
pub struct Payload {
    /// Principal that issued the JWT. In this context, user public key.
    pub iss: Option<String>,
    /// Unique identifier for the JWT.
    pub jti: Option<String>,
    /// Time at which the JWT was issued.
    pub iat: Option<u64>,
    /// Expiration time on or after which the JWT MUST NOT be accepted for processing.
    pub exp: Option<u64>,
    // todo(ludo): add description
    #[serde(rename = "gaiaChallenge")]
    pub gaia_challenge: Option<String>,
    /// Add some salt to limit replay attacks
    pub salt: Option<Salt>,
    // todo(ludo): add description
    #[serde(rename = "hubUrl")]
    pub hub_url: Option<String>,
}

impl Payload {

    pub fn new(app_public_key: String, hub_url: String, gaia_challenge: String) -> Self {
        
        // Generate UUID
        let uuid = Uuid::new_v4().to_string();

        // Generate Salt
        // let mut rng = thread_rng();
        // let mut salt = [0u8; 16];
        // rng.fill(&mut salt);
        // let salt_hex = hex::encode(&salt);

        Self {
            jti: Some(uuid),
            // todo(ludo): set issued at
            iat: Some(0),
            // todo(ludo): set expiration
            exp: Some(0),
            iss: Some(app_public_key),
            salt: None,
            hub_url: Some(hub_url),
            gaia_challenge: Some(gaia_challenge)
        }
    }
}
