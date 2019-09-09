use drivers::{
    driver::*
};
use primitives::{
    file::{
        File,
        TearUpResult,
        TearDownResult,
        ListFilesResult,
        CreateFileResult,
        ReadFileResult,
        UpdateFileResult,
        DeleteFileResult
    },
    errors::Error
};

use rusoto_core::{Region};
use rusoto_credential::{
    ProvideAwsCredentials, 
    ProfileProvider
};
use rusoto_s3::{
    S3, 
    S3Client, 
    ListObjectsRequest,
    PutObjectRequest,
    GetObjectRequest,
    DeleteObjectRequest
};
use futures::future::Future;
use std::path::Path;

pub struct S3Driver {
}

impl StorageDriver for S3Driver {
        
    fn tear_up(params: TearUpParams) -> Result<TearUpResult, Error> {
        Err(Error::Unknown)
    }

    fn tear_down(params: TearDownParams) -> Result<TearDownResult, Error> {
        Err(Error::Unknown)
    }

    fn list_files(params: ListFilesParams) -> Result<ListFilesResult, Error> {

        let credentials = ProfileProvider::new()
            .unwrap()
            .credentials()
            .wait()
            .unwrap();

        let client = S3Client::new(Region::UsEast1);
        let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

        let mut list_request = ListObjectsRequest::default();
        list_request.bucket = bucket.to_string();

        let response = client.list_objects(list_request).sync();

        match response {
            Ok(res) => {
                let mut files = vec![];
                if let Some(entries) = res.contents {
                    for entry in entries.iter() {
                        let key = entry.key.clone().unwrap_or_else(|| {
                            // Err(Error::Unknown);
                            panic!("Error");
                        });
                        let updated_at = entry.last_modified.clone();
                        let path = Path::new(&key);
                        let name = path.file_name().clone().unwrap_or_else(|| {
                            // Err(Error::Unknown);
                            panic!("Error");
                        });
                        files.push(File {
                            name: name.to_os_string(),
                            path: path.as_os_str().to_os_string(),
                            storage_top_level: params.path.clone(),
                            updated_at,
                            content: None,
                            content_type: None,
                            content_length: None
                        });
                    }
                }
                Ok(ListFilesResult {
                    files
                })
            }
            Err(e) => {
                println!("{:}", e);
                Err(Error::Unknown)
            }
        }
    }

    fn create_file(params: CreateFileParams) -> Result<CreateFileResult, Error> {

        let credentials = ProfileProvider::new()
            .unwrap()
            .credentials()
            .wait()
            .unwrap();

        let client = S3Client::new(Region::UsEast1);
        let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

        let mut put_request = PutObjectRequest::default();
        put_request.bucket = bucket.to_string();

        let response = client.put_object(put_request).sync();

        match response {
            Ok(entry) => {
                let name = params.name.clone();
                let path = params.path.clone();
                let storage_top_level = params.storage_top_level.clone();

                // let updated_at = entry.last_modified.clone();
                // let content_type = entry.content_type.clone();
                // let content_length = entry.content_length.clone();

                let file = File {
                    name,
                    path,
                    storage_top_level,
                    updated_at: None,
                    content: None,
                    content_type: None,
                    content_length: None
                };
                Ok(CreateFileResult {
                    file
                })
            }
            Err(e) => {
                println!("{:}", e);
                Err(Error::Unknown)
            }
        }

    }

    fn read_file(params: ReadFileParams) -> Result<ReadFileResult, Error> {
        let credentials = ProfileProvider::new()
            .unwrap()
            .credentials()
            .wait()
            .unwrap();

        let client = S3Client::new(Region::UsEast1);
        let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

        let mut get_request = GetObjectRequest::default();
        get_request.bucket = bucket.to_string();

        let response = client.get_object(get_request).sync();

        match response {
            Ok(entry) => {
                let name = params.file.name.clone();
                let path = params.file.path.clone();
                let storage_top_level = params.file.storage_top_level.clone();

                let updated_at = entry.last_modified.clone();
                let content_type = entry.content_type.clone();
                let content_length = entry.content_length.clone();

                let file = File {
                    name,
                    path,
                    storage_top_level,
                    updated_at,
                    content: None,
                    content_type,
                    content_length
                };
                Ok(ReadFileResult {
                    file
                })
            }
            Err(e) => {
                println!("{:}", e);
                Err(Error::Unknown)
            }
        }
    }

    fn update_file(params: UpdateFileParams) -> Result<UpdateFileResult, Error> {
        let credentials = ProfileProvider::new()
            .unwrap()
            .credentials()
            .wait()
            .unwrap();

        let client = S3Client::new(Region::UsEast1);
        let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

        let mut put_request = PutObjectRequest::default();
        put_request.bucket = bucket.to_string();

        let response = client.put_object(put_request).sync();

        match response {
            Ok(entry) => {
                Ok(UpdateFileResult {
                    file: params.file
                })
            }
            Err(e) => {
                println!("{:}", e);
                Err(Error::Unknown)
            }
        }
    }
    
    fn delete_file(params: DeleteFileParams) -> Result<DeleteFileResult, Error> {
        let credentials = ProfileProvider::new()
            .unwrap()
            .credentials()
            .wait()
            .unwrap();

        let client = S3Client::new(Region::UsEast1);
        let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

        let mut delete_request = DeleteObjectRequest::default();
        delete_request.bucket = bucket.to_string();

        let response = client.delete_object(delete_request).sync();

        match response {
            Ok(entry) => {
                Ok(DeleteFileResult {
                    file: params.file
                })
            }
            Err(e) => {
                println!("{:}", e);
                Err(Error::Unknown)
            }
        }
    }

    // fn lock_file(params: LockFileParams) -> Result<LockFileResult, Error> {
    //     let credentials = ProfileProvider::new()
    //         .unwrap()
    //         .credentials()
    //         .wait()
    //         .unwrap();

    //     let client = S3Client::new(Region::UsEast1);
    //     let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

    //     let mut lock_request = DeleteObjectRequest::default();
    //     lock_request.bucket = bucket.to_string();

    //     Ok(LockFileResult {
    //         file: params.file,
    //         token: vec![]
    //     })
    // }

    // fn unlock_file(params: UnlockFileParams) -> Result<UnlockFileResult, Error> {
    //     let credentials = ProfileProvider::new()
    //         .unwrap()
    //         .credentials()
    //         .wait()
    //         .unwrap();

    //     let client = S3Client::new(Region::UsEast1);
    //     let bucket = format!("gaia-faas-attachmentsbucket-pyy40p399aff");

    //     let mut unlock_request = DeleteObjectRequest::default();
    //     unlock_request.bucket = bucket.to_string();

    //     Ok(UnlockFileResult {
    //         file: params.file
    //     })
    // }
}

