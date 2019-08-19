use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use fuse::{FileType, FileAttr};
use time::Timespec;

const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };

pub struct FileMap {
    /// todo(ludo): add description
    next_inode: u64,
    /// Keep track of files: (u64, )
    pub files: HashMap<u64, (OsString, FileAttr)>,
    /// Keep track of direto
    pub directory_content: HashMap<u64, Vec<u64>>,
    /// Lookup (parent, name) -> ino
    pub lookup: HashMap<(u64, OsString), u64>
}

impl FileMap {
    pub fn new() -> Self {
        let mut file_map = Self {
            next_inode: 1,
            files: HashMap::new(),
            directory_content: HashMap::new(),
            lookup: HashMap::new()
        };
        file_map.insert_root();
        file_map
    }

    pub fn insert_root(&mut self) {

        let root_attr = self.new_file(FileType::Directory);
        let ino = root_attr.ino;
        
        self.files.insert(ino, (OsString::from(""), root_attr));

        self.directory_content.insert(ino, Vec::new());
    }


    pub fn new_regular_file(&mut self, parent: u64, name: &OsStr) -> FileAttr {

        let file_attr = self.new_file(FileType::RegularFile);
        let ino = file_attr.ino;
        
        self.files.insert(ino, (name.to_os_string(), file_attr));

        let siblings = self.directory_content.get_mut(&parent).unwrap();
        siblings.push(ino);

        self.lookup.insert((parent, name.to_os_string()), ino);

        file_attr
    }

    pub fn new_directory(&mut self, parent: u64, name: &OsStr) -> FileAttr {

        let file_attr = self.new_file(FileType::Directory);
        let ino = file_attr.ino;
        
        self.files.insert(ino, (name.to_os_string(), file_attr));

        self.directory_content.insert(ino, Vec::new());

        let siblings = self.directory_content.get_mut(&parent).unwrap();
        siblings.push(ino);

        self.lookup.insert((parent, name.to_os_string()), ino);

        file_attr
    }

    pub fn new_file(&mut self, kind: FileType) -> FileAttr {
        let ino = self.next_inode;
        self.next_inode += 1;

        FileAttr {
            ino,
            size: 512,
            blocks: 1,
            atime: CREATE_TIME,
            mtime: CREATE_TIME,
            ctime: CREATE_TIME,
            crtime: CREATE_TIME,
            kind,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        }
    }
}
