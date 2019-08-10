#![feature(async_await)]

use async_trait::async_trait;

pub mod list_files;

pub struct AuthenticationToken {
    /// Token's value - base64 encoded.
    value: String
}

impl AuthenticationToken {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string()
        }
    }
}

#[derive(Debug)]
pub enum AuthenticationError {
    TimeOutError
}

pub type AuthenticationResult = Result<AuthenticationToken, AuthenticationError>;

#[async_trait]
pub trait AuthenticationDelegate {
    /// Async method. Returns a future, returning itself an AuthenticationResult.
    async fn get_authorization_token(&self) -> AuthenticationResult;
}
