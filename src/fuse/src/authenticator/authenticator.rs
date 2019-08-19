use async_trait::async_trait;
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

pub struct Authenticator {
    pub authentication_token: String
}

impl Authenticator {
    pub fn new(authentication_token: String) -> Self {
        Self {
            authentication_token
        }
    }
}

#[async_trait]
impl AuthenticationDelegate for Authenticator {

    async fn get_authorization_token(&self) -> AuthenticationResult {
        Ok(AuthenticationToken::new(&self.authentication_token))
    }
}

