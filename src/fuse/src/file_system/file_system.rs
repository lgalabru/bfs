extern crate env_logger;
use std::ffi::{OsStr, OsString};
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Request, ReplyData, ReplyEntry, ReplyEmpty, ReplyOpen, ReplyAttr, ReplyDirectory};
use crate::file_system::{SyncEngine};

use primitives::{
    file::{
        File, 
        ListFilesResult
    },
    errors::Error
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
    sync_engine: SyncEngine
}

impl FS {

    pub fn new(sync_engine: SyncEngine) -> Self {
        Self {
            sync_engine
        }
    }
}

impl fuse::Filesystem for FS {

    // fn init(&mut self, _req: &Request) -> Result<(), c_int> {
    // }

    /// Open a directory.
    fn opendir(&mut self, _req: &Request, ino: u64, _flags: u32, reply: ReplyOpen) {
        reply.opened(1, 0);
    }

    /// Read directory.
    fn readdir(&mut self, req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {

        if ino < 1 {
            reply.error(ENOENT);
            return;
        }


        let mut curr_offs = offset + 1;
        match self.sync_engine.file_map.directory_entries.get(&ino) {
            Some(entries) => {
                for file_ino in entries.iter().skip(offset as usize) {                    
                    let (name, attrs) = self.sync_engine.file_map.files.get(file_ino).unwrap(); // todo(ludo): fix unwrap
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
        reply.ok();
    }

    /// Look up a directory entry by name and get its attributes.
    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent < 1 {
            reply.error(ENOENT);
            return
        }

        let key = (parent, name.to_os_string());

        let ino = match self.sync_engine.file_map.lookup.get(&key) {
            Some(ino) => ino,
            None => {
                reply.error(ENOENT);
                return
            }
        };

        let (_, attrs) = match self.sync_engine.file_map.files.get(&ino) {
            Some(res) => res,
            None => {
                reply.error(ENOENT);
                return
            }
        }; 

        reply.entry(&TTL, &attrs, 0);
    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {

        match ino {
            1 => reply.attr(&TTL, &BFS_DIR_ATTR),
            _ => {
                let (_, attrs) = self.sync_engine.file_map.files.get(&ino).unwrap(); // todo(ludo): fix unwrap
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
