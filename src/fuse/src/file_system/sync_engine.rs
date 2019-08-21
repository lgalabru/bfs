use std::collections::{HashMap, VecDeque};
use std::ffi::{OsString};
use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};
use fuse::{FileType, FileAttr};
use crate::file_system::{FileMap};
use crate::authenticator::{Authenticator};
use bfs_commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
};

pub struct Request {
    endpoint: String,
    path: String,
}

pub struct SyncEngine {
    authenticator_map: HashMap<u64, (OsString, String, Authenticator)>,
    pub file_map: FileMap,
    pub request_queue: VecDeque<Request>
}

impl SyncEngine {

    pub fn new() -> Self {
        Self {
            authenticator_map: HashMap::new(),
            file_map: FileMap::new(),
            request_queue: VecDeque::new(),
        }
    }

    pub async fn register_endpoint(&mut self, name: OsString, url: String, authorization_token: String) {
        let attr = self.file_map.register_directory(1, &name);
        self.authenticator_map.insert(attr.ino, (name, url.clone(), Authenticator::new(authorization_token))); // todo(ludo): fix unwrap
        self.sync_dir(attr, String::from("/"), url).await;
    }

    /// todo(ludo): sort out the str / osstr / cloned situation
    pub async fn sync_dir(&mut self, dir_attr: FileAttr, path: String, endpoint_url: String) {

        let (_, _, authenticator) = self.authenticator_map.get(&dir_attr.ino).unwrap();
        let builder = ListFilesCommandBuilder::new(
            OsString::from(path.clone()),
            authenticator 
        );
        let res = builder.run().await;
        let command = res.unwrap();

        let result = bfs_http_client::list_files(&endpoint_url, &command.authorization_token, &path);

        match result {
            Ok(payload) => {
                for entry in payload.entries {
                    let components: Vec<&str> = entry.split("/").collect();
                    let file_index: usize = components.len() - 1;
                    let mut parent_ino = dir_attr.ino;

                    for (i, comp) in components.iter().enumerate() {
                        if i == file_index {
                            self.file_map.register_regular_file(parent_ino, &OsString::from(comp.clone()));
                        } else {
                            let attr = self.file_map.register_directory(parent_ino, &OsString::from(comp.clone()));
                            parent_ino = attr.ino;
                        }
                    }
                }
            },
            Err(e) => println!("Error, {:?}", e)
        }
    }
}
