use async_trait::async_trait;

use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};

pub struct Authenticator;

impl Authenticator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthenticationDelegate for Authenticator {
    async fn get_authorization_token(&self) -> AuthenticationResult {
        error!("Authentication in progress...");
        Ok(AuthenticationToken::new("0x1234567890ABCDEF"))
    }
}
