extern crate env_logger;
use std::ffi::{OsStr, OsString};
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Request, ReplyData, ReplyEntry, ReplyEmpty, ReplyOpen, ReplyAttr, ReplyDirectory};
use std::collections::HashMap;
use tokio::runtime::Runtime;
use crate::authenticator::LocalAuthenticator;
use crate::file_system::{FileAdapter, FileMap};

use primitives::{
    file::{
        File, 
        ListFilesResult
    },
    errors::Error
};

use bfs_commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
};

const TTL: Timespec = Timespec { sec: 1, nsec: 0 };                     // 1 second
const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56

const BFS_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const TEMP_CONTENT: &'static str = "Hello World!\n";

pub struct FS {
    // cached_lookup: HashMap<(u32, OsString), FileAttr>,
    // cached_getattr: HashMap<(u32, u64), FileAttr>,
    authenticator: LocalAuthenticator,
    file_map: FileMap
}

impl FS {

    pub fn new(authenticator: LocalAuthenticator, file_map: FileMap) -> Self {
        Self {
            authenticator,
            file_map
        }
    }

    async fn list_files(&mut self, prefix_path: &str) -> Result<ListFilesResult, Error>{
        let builder = ListFilesCommandBuilder::new(
            OsString::from(prefix_path),
            &self.authenticator 
        );

        let res = builder.run().await;

        let command = res.unwrap();

        let handler = ListFilesCommandHandler::new(&command);
        handler.run()
    }
}

impl fuse::Filesystem for FS {

    // fn init(&mut self, _req: &Request) -> Result<(), c_int> {
    // }

    /// Open a directory.
    fn opendir(&mut self, _req: &Request, ino: u64, _flags: u32, reply: ReplyOpen) {
        error!("opendir: {}", ino);
        reply.opened(1, 0);
    }

    /// Read directory.
    fn readdir(&mut self, req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        error!("readdir: {} - {:?}", ino, req);

        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let mut curr_offs = offset + 1;
        match self.file_map.directory_content.get(&ino) {
            Some(entries) => {
                for file_ino in entries.iter().skip(offset as usize) {                    
                    let (name, attrs) = self.file_map.files.get(file_ino).unwrap(); // todo(ludo): fix unwrap
                    if reply.add(*file_ino, curr_offs, attrs.kind , &name.clone()) {
                        break;
                    } else {
                        curr_offs += 1;
                    }
                }
                reply.ok();
            }
            None => {
                reply.error(ENOENT);
            }
        };
    }

    /// Release an open directory.
    fn releasedir(&mut self, _req: &Request, ino: u64, _fh: u64, _flags: u32, reply: ReplyEmpty) {
        error!("releasedir: {}", ino);
        reply.ok();
    }

    /// Look up a directory entry by name and get its attributes.
    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        error!("lookup: {:?} {:?} {:?}", name, req, parent);

        if parent != 1 {
            reply.error(ENOENT);
            return
        }

        let key = (parent, name.to_os_string());

        let ino = match self.file_map.lookup.get(&key) {
            Some(ino) => ino,
            None => {
                reply.error(ENOENT);
                return
            }
        };
        error!("lookup: {:?} {:?}", key, ino);

        let (_, attrs) = match self.file_map.files.get(&ino) {
            Some(res) => res,
            None => {
                reply.error(ENOENT);
                return
            }
        }; 
        error!("lookup: {:?}", attrs);

        reply.entry(&TTL, &attrs, 0);
    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        error!("getattr: {} - {:?}", ino, req);

        match ino {
            1 => reply.attr(&TTL, &BFS_DIR_ATTR),
            _ => {
                let (_, attrs) = self.file_map.files.get(&ino).unwrap(); // todo(ludo): fix unwrap
                reply.attr(&TTL, &attrs);
            }
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, reply: ReplyData) {
        if ino == 2 {
            reply.data(&TEMP_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }
}
