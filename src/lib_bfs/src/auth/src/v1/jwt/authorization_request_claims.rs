use serde::{Deserialize, Serialize};
use crate::v1::types::{AuthScope};
use uuid::Uuid;

/// JWT Claims Set for "authentication response tokens"
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
    pub scopes: Option<Vec<AuthScope>>,
    // todo(ludo): add description, should serialize to domain_name (?)
    pub app_domain: Option<String>,
    // todo(ludo): add description
    pub manifest_uri: Option<String>,
    // todo(ludo): add description
    pub redirect_uri: Option<String>,
    // todo(ludo): add description
    pub do_not_include_profile: Option<bool>,
    // todo(ludo): add description
    pub supports_hub_url: Option<bool>,
    // todo(ludo): add description
    pub version: Option<String>,
}

impl Payload {

    pub fn new(app_domain: String, 
               do_not_include_profile: bool, 
               manifest_uri: String, 
               redirect_uri: String, 
               version: String, 
               scopes: Vec<AuthScope>,
               supports_hub_url: bool) -> Self {

        // Generate UUID
        let uuid = Uuid::new_v4().to_string();
        
        Self {
            jti: Some(uuid),
            iat: Some(0),
            exp: Some(0),
            iss: None,
            app_domain: Some(app_domain),
            manifest_uri: Some(manifest_uri),
            redirect_uri: Some(redirect_uri),
            version: Some(version),
            do_not_include_profile: Some(do_not_include_profile),
            supports_hub_url: Some(supports_hub_url),
            scopes: Some(scopes)
        }
    }
}
