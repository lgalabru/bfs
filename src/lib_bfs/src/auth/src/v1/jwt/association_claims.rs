use serde::{Deserialize, Serialize};

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
    /// Add some salt to limit replay attacks
    pub salt: Option<String>,
    /// App public key
    pub childToAssociate: Option<String>
}

impl Payload {

    pub fn new(user_public_key: String, app_public_key: String, salt: String) -> Self {
        Self {
            jti: Some("1".to_string()),
            iat: Some(0),
            exp: Some(0),
            iss: Some(user_public_key),
            childToAssociate: Some(app_public_key),
            salt: Some(salt)
        }
    }
}
