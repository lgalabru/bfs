use async_trait::async_trait;
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};

#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub struct LocalAuthenticator {
    authentication_tokens: HashMap<String, String>,
    active_app_domain: Option<String>
}

impl LocalAuthenticator {
    pub fn new(authentication_tokens: HashMap<String, String>) -> Self {
        Self {
            authentication_tokens,
            active_app_domain: None
        }
    }
}

#[async_trait]
impl AuthenticationDelegate for LocalAuthenticator {

    async fn get_authorization_token(&self) -> AuthenticationResult {
        let app_domain = match &self.active_app_domain {
            Some(app_domain) => app_domain,
            None => { return Err(AuthenticationError::Unknown); }
        };
        let token = self.authentication_tokens.get(app_domain).unwrap(); // todo(ludo): fix unwrap
        
        Ok(AuthenticationToken::new(token))
    }
}

