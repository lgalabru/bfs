use async_trait::async_trait;
use std::collections::HashMap;
use std::ffi::{OsString};
use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};
use crate::file_system::{FileMap};

pub struct SyncEngine {
    authentication_tokens: HashMap<String, String>,
    pub file_map: FileMap
}

impl SyncEngine {

    pub fn new() -> Self {
        Self {
            authentication_tokens: HashMap::new(),
            file_map: FileMap::new()
        }
    }

    pub fn register_endpoint(&mut self, name: OsString, url: String, authorization_token: String) {
        self.file_map.new_directory(1, &name);
        self.authentication_tokens.insert(url, authorization_token);
    }
}

#[async_trait]
impl AuthenticationDelegate for SyncEngine {

    async fn get_authorization_token(&self) -> AuthenticationResult {
        // let app_domain = match &self.active_app_domain {
        //     Some(app_domain) => app_domain,
        //     None => { return Err(AuthenticationError::Unknown); }
        // };
        // let token = self.authentication_tokens.get(app_domain).unwrap(); // todo(ludo): fix unwrap
        
        Ok(AuthenticationToken::new(""))
    }
}