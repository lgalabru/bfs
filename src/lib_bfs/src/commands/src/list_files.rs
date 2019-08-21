use std::ffi::{OsString};
use drivers::{
    driver::{
        StorageDriver,
        ListFilesParams
    }
};

use primitives::{
    file::ListFilesResult,
    errors::Error
};

use drivers_aws::s3::S3Driver;
use crate::{AuthenticationDelegate};

#[derive(Debug)]
pub enum CommandBuildError {
    TimeOutError
}

pub type CommandBuildResult = Result<ListFilesCommand, CommandBuildError>;


pub struct ListFilesCommand {

    pub path: OsString, 
    
    pub page: Option<u32>,

    pub authorization_token: String,
}

pub struct ListFilesCommandBuilder<'a> {
    
    pub path: OsString, 
    
    pub page: Option<u32>,

    authentication_delegate: &'a (dyn AuthenticationDelegate + 'a)
}

impl <'a> ListFilesCommandBuilder <'a> {

    pub fn new(path: OsString, authentication_delegate: &'a dyn AuthenticationDelegate) -> ListFilesCommandBuilder<'a> {
        ListFilesCommandBuilder {
            path,
            page: None,
            authentication_delegate
        }
    }

    pub async fn run(&self) -> CommandBuildResult {
        let result = self.authentication_delegate.get_authorization_token().await;
        if let Err(_) = result {
            return Err(CommandBuildError::TimeOutError)
        }
        let authorization_token = result.unwrap().value;

        Ok(ListFilesCommand {
            path: self.path.clone(),
            page: self.page,
            authorization_token,
        })
    }
}

pub struct ListFilesCommandHandler<'a> {
    pub command: &'a ListFilesCommand,
}

impl <'a> ListFilesCommandHandler<'a> {

    pub fn new(command: &'a ListFilesCommand) -> ListFilesCommandHandler {
        ListFilesCommandHandler {
            command
        }
    }

    pub fn run(&self) -> Result<ListFilesResult, Error> {
        println!("Verifying token {}...", self.command.authorization_token);

        // Check authorization
        println!("Listing files in {:?}", self.command.path);

        let params = ListFilesParams {
            path: self.command.path.clone(),
            page: self.command.page
        };
        S3Driver::list_files(params)
    }
}

//   validate(address: string, requestHeaders: RequestHeaders, oldestValidTokenTimestamp?: number) {
//     const signingAddress = validateAuthorizationHeader(requestHeaders.authorization,
//                                                        this.serverName, address,
//                                                        this.requireCorrectHubUrl,
//                                                        this.validHubUrls, 
//                                                        oldestValidTokenTimestamp)

//     if (this.whitelist && !(this.whitelist.includes(signingAddress))) {
//       throw new ValidationError(`Address ${signingAddress} not authorized for writes`)
//     }
//   }