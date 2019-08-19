mod file_system;
mod file_map;

pub use self::file_system::{FS};
pub use self::file_map::{FileMap};

use fuse::{FileType, FileAttr};
use primitives::file::File;
use time::Timespec;


const CREATE_TIME: Timespec = Timespec { sec: 1381237736, nsec: 0 };    // 2013-10-08 08:56

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

