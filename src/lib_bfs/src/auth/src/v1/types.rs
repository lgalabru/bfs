use serde::{Deserialize, Serialize};

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
