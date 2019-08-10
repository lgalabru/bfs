#![feature(async_await)]
#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use libc::ENOENT;
use time::Timespec;
use fuse::{FileType, FileAttr, Filesystem, Request, ReplyData, ReplyEntry, ReplyAttr, ReplyDirectory};
use log::Level;

use async_trait::async_trait;
use tokio::runtime::Runtime;

use primitives::{
    file::{
        File, 
        ListFilesResult
    },
    errors::Error
};

use commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
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

struct FileAdapter;

impl FileAdapter {

    fn convert(file: &File, inode: u64) -> FileAttr {
        FileAttr {
            ino: inode,
            size: 13,
            blocks: 1,
            atime: CREATE_TIME,
            mtime: CREATE_TIME,
            ctime: CREATE_TIME,
            crtime: CREATE_TIME,
            kind: FileType::RegularFile,
            perm: 0o644,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
        }
    }
}

struct LocalAuthenticator;

impl LocalAuthenticator {
    pub fn new() -> LocalAuthenticator {
        LocalAuthenticator {}
    }
}

#[async_trait]
impl AuthenticationDelegate for LocalAuthenticator {
    async fn get_authorization_token(&self) -> AuthenticationResult {
        println!("Authentication in progress...");
        Ok(AuthenticationToken::new("0x1234567890ABCDEF"))
    }
}

struct BFS {
    cached_lookup: HashMap<(u32, OsString), FileAttr>,
    cached_getattr: HashMap<(u32, u64), FileAttr>,
    authentication_delegate: LocalAuthenticator,
}

impl BFS {

    async fn list_files(&mut self, prefix_path: &str) -> Result<ListFilesResult, Error>{
        let builder = ListFilesCommandBuilder::new(
            OsString::from(prefix_path),
            &self.authentication_delegate 
        );


        let res = builder.run().await;

        let command = res.unwrap();

        let handler = ListFilesCommandHandler::new(&command);
        handler.run()
    }
}

impl Filesystem for BFS {

    // fn init(&mut self, _req: &Request) -> Result<(), c_int> {
    // }

    fn readdir(&mut self, req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        error!("readdir: {} - {}", ino, req.uid());

        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let rt = Runtime::new().unwrap();
        rt.block_on(async {

            let prefix_path = "/";

            let result = self.list_files(prefix_path).await;
        
            if let Err(_e) = result {
                // Improve error output
                reply.error(ENOENT);
                return
            }
            let files = result.unwrap().files;

            if offset == 0 {
                reply.add(1, 0, FileType::Directory, ".");
                reply.add(1, 1, FileType::Directory, "..");
            }
            
            let to_skip = if offset == 0 { offset } else { offset + 1 } as usize;
            for (i, file) in files.into_iter().enumerate().skip(to_skip) {
                let index = i + 2;
                reply.add(
                    index as u64, 
                    index as i64, 
                    FileType::RegularFile, 
                    file.name.as_os_str());
                let file_attr = FileAdapter::convert(&file, index as u64);
                self.cached_lookup.insert((req.uid(), file.name), file_attr);
                self.cached_getattr.insert((req.uid(), index as u64), file_attr);
            }

            error!("readdir: {} files", self.cached_lookup.len());
            reply.ok();
        });
    }

    fn lookup(&mut self, req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        error!("lookup: {:?}", name);

        if parent != 1 {
            reply.error(ENOENT);
            return
        }

        let wrapped_res = &self.cached_lookup.get(&(req.uid(), name.to_os_string()));
        if let None = wrapped_res {
            // Improve error output
            reply.error(ENOENT);
            return
        }
        let file_attr = wrapped_res.unwrap();
        reply.entry(&TTL, &file_attr, 0);
    }

    fn getattr(&mut self, req: &Request, ino: u64, reply: ReplyAttr) {
        error!("getattr: {}", ino);

        match ino {
            1 => reply.attr(&TTL, &BFS_DIR_ATTR),
            _ => {
                let wrapped_res = &self.cached_getattr.get(&(req.uid(), ino));
                if let None = wrapped_res {
                    // Improve error output
                    reply.error(ENOENT);
                    return
                }
                let file_attr = wrapped_res.unwrap();
                reply.attr(&TTL, &file_attr);
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

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let bfs = BFS {
        cached_lookup: HashMap::new(),
        cached_getattr: HashMap::new(),
        authentication_delegate: LocalAuthenticator::new()
    };
    fuse::mount(bfs, &mountpoint, &options).unwrap();
}