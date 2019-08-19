use std::collections::{HashMap, VecDeque};
use std::ffi::{OsString};
use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};
use fuse::{FileType, FileAttr};
use tokio::runtime::Runtime;
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
            request_queue: VecDeque::new()
        }
    }

    pub fn register_endpoint(&mut self, name: OsString, url: String, authorization_token: String) {
        let attr = self.file_map.register_directory(1, &name);
        self.authenticator_map.insert(attr.ino, (name, url, Authenticator::new(authorization_token))); // todo(ludo): fix unwrap
        let mut rt = Runtime::new().unwrap();
        rt.block_on(async {
            self.sync_dir(attr, OsString::from("/"));
        });
    }

    pub async fn sync_dir(&mut self, dir_attr: FileAttr, path: OsString) {

        let (_, _, authenticator) = self.authenticator_map.get(&dir_attr.ino).unwrap();
        let builder = ListFilesCommandBuilder::new(
            path,
            authenticator 
        );
        let res = builder.run().await;
        let command = res.unwrap();

        let handler = ListFilesCommandHandler::new(&command);
        match handler.run() {
            Ok(list_files_result) => {
                println!("{:?}", list_files_result);

                for file in list_files_result.files {
                    self.file_map.register_regular_file(dir_attr.ino, &file.name);
                }
            },
            Err(e) => println!("Error, {:?}", e)
        }
    }
}
