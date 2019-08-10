use std::ffi::OsString;

#[derive(Debug)]
pub struct File {

    pub name: OsString,

    pub path: OsString,

    pub storage_top_level: OsString,

    pub updated_at: Option<String>,

    pub content: Option<Vec<u8>>,

    pub content_type: Option<String>,

    pub content_length: Option<i64>
}

impl File {
    pub fn new(name: OsString, path: OsString, storage_top_level: OsString) -> File {
        File {
            name,
            path,
            storage_top_level,
            updated_at: None,
            content: None,
            content_type: None,
            content_length: None
        }
    }
}

#[derive(Debug)]
pub struct TearUpResult;

#[derive(Debug)]
pub struct TearDownResult {
}

#[derive(Debug)]
pub struct ListFilesResult {
    pub files: Vec<File>
}

#[derive(Debug)]
pub struct CreateFileResult {
    pub file: File
}

#[derive(Debug)]
pub struct ReadFileResult {
    pub file: File
}

#[derive(Debug)]
pub struct UpdateFileResult {
    pub file: File
}

#[derive(Debug)]
pub struct DeleteFileResult {
    pub file: File
}