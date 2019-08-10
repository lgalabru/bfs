pub mod association_claims;
pub mod authorization_claims;

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

impl Header {
    pub fn new() -> Header {
        Header {
            alg: Some("ES256K".to_string()),
            typ: Some("JWT".to_string())
        }
    }
}