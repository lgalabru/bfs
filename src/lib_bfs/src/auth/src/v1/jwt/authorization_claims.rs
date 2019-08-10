use serde::{Deserialize, Serialize};

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
    /// Principal that is the subject of the JWT.
    pub sub: Option<u64>,
    /// App specific:
    pub scopes: Option<Vec<AuthScope>>,
    /// App specific:
    pub gaiaChallenge: Option<String>,
    /// Add some salt to limit replay attacks
    pub salt: Option<Salt>,
    /// App specific:
    pub hubUrl: Option<String>,
    /// App specific:
    pub associationToken: Option<String>,
    /// App specific: Token signed by one of the whitelisted addresses on this server. 
    /// This method checks a given associationToken and verifies that it authorizes the "outer"
    /// JWT's address (`bearerAddress`).
    pub childToAssociate: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct AuthScope {
    pub scope: Option<String>,
    pub domain: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Salt {
    pub r#type: Option<String>,
    pub data: Option<Vec<u8>>
}

// export const AuthScopes = [
//   'putFile',
//   'putFilePrefix',
//   'deleteFile',
//   'deleteFilePrefix'
// ]
