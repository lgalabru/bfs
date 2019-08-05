use serde::{Deserialize, Serialize};

/// JSON Web Token (JWT) is a compact, URL-safe means of representing
/// claims to be transferred between two parties.  The claims in a JWT
/// are encoded as a JSON object that is used as the payload of a JSON
/// Web Signature (JWS) structure or as the plaintext of a JSON Web
/// Encryption (JWE) structure, enabling the claims to be digitally
/// signed or integrity protected with a Message Authentication Code
/// (MAC) and/or encrypted.
/// https://tools.ietf.org/html/rfc7519


#[derive(Serialize, Deserialize)]
pub struct Header {
    /// Algorithms used (https://tools.ietf.org/html/rfc7518#section-3)
    pub alg: Option<String>,
    /// Media type (http://www.iana.org/assignments/media-types/media-types.xhtml) of this complete JWT.
    pub typ: Option<String>,
}

/// The JWT Claims Set (Payload part) represents a JSON object whose members are the
/// claims conveyed by the JWT.  The Claim Names within a JWT Claims Set
/// MUST be unique;
/// https://tools.ietf.org/html/rfc7519#section-4
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
    /// App specific:
    pub salt: Option<String>,
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

// export const AuthScopes = [
//   'putFile',
//   'putFilePrefix',
//   'deleteFile',
//   'deleteFilePrefix'
// ]
