use hex;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    /// Add some salt to limit replay attacks - look at signing oracles
    pub salt: Option<String>,
    /// App public key to associate
    #[serde(rename = "childToAssociate")]
    pub child_to_associate: Option<String>,
}

impl Payload {
    pub fn new(user_public_key: String, app_public_key: String) -> Self {
        // Generate UUID
        let uuid = Uuid::new_v4().to_string();

        // Generate Salt
        let mut rng = thread_rng();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        let salt_hex = hex::encode(&salt);

        Self {
            jti: Some(uuid),
            // todo(ludo): set issued at
            iat: Some(0),
            // todo(ludo): set expiration
            exp: Some(0),
            iss: Some(user_public_key),
            child_to_associate: Some(app_public_key),
            salt: Some(salt_hex),
        }
    }
}
