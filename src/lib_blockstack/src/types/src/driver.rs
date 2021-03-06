use super::{
    errors::Error,
    file::{
        CreateFileResult, DeleteFileResult, File, ListFilesResult, ReadFileResult, TearDownResult,
        TearUpResult, UpdateFileResult,
    },
};
use std::ffi::OsString;

#[derive(Debug)]
pub struct TearUpParams;

#[derive(Debug)]
pub struct TearDownParams {}

#[derive(Debug)]
pub struct ListFilesParams {
    pub path: OsString,

    pub page: Option<u32>,
}

#[derive(Debug)]
pub struct CreateFileParams {
    pub name: OsString,

    pub path: OsString,

    pub storage_top_level: OsString,

    pub content: Vec<u8>,

    pub content_type: String,

    pub content_length: i64,
}

#[derive(Debug)]
pub struct ReadFileParams {
    pub file: File,
}

#[derive(Debug)]
pub struct UpdateFileParams {
    pub file: File,
}

#[derive(Debug)]
pub struct DeleteFileParams {
    pub file: File,
}

pub trait StorageDriver {
    fn tear_up(params: TearUpParams) -> Result<TearUpResult, Error>;

    fn tear_down(params: TearDownParams) -> Result<TearDownResult, Error>;

    fn list_files(params: ListFilesParams) -> Result<ListFilesResult, Error>;

    fn create_file(params: CreateFileParams) -> Result<CreateFileResult, Error>;

    fn read_file(params: ReadFileParams) -> Result<ReadFileResult, Error>;

    fn update_file(params: UpdateFileParams) -> Result<UpdateFileResult, Error>;

    fn delete_file(params: DeleteFileParams) -> Result<DeleteFileResult, Error>;
}
