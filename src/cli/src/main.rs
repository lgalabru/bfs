#![feature(async_await)]

use std::ffi::OsString;
use async_trait::async_trait;
use tokio;

use clap::{
    Arg, 
    App,
    SubCommand
};

use commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};

// use termcolor::{
//     Color, 
//     ColorChoice, 
//     ColorSpec, 
//     StandardStream, 
//     WriteColor
// };

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

#[tokio::main]
async fn main() {
    let matches = App::new("bfs-cli")
        .about("Blockstack File System")
        .version("1.0")
        .author("Ludovic Galabru")
        .subcommand(SubCommand::with_name("ls")
            .aliases(&["list", "dir"])
            .about("List directory contents")
            .arg(Arg::with_name("path")
                .help("The path to explore")
                .required(false)))
        .subcommand(SubCommand::with_name("rm")
            .aliases(&["delete", "remove"])
            .about("Remove directory entries")
            .arg(Arg::with_name("path")
                .help("The path to explore")
                .required(true)))
        .subcommand(SubCommand::with_name("mkdir")
            .about("Make directories")
            .arg(Arg::with_name("path")
                .help("The path to explore")
                .required(true)))
        .subcommand(SubCommand::with_name("cat")
            .about("Print file")
            .arg(Arg::with_name("path")
                .help("The path to explore")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("ls", Some(clone_matches)) => {
            let prefix_path = match clone_matches.value_of("path") {
                Some(path) => path,
                None => "/"
            };
            list_files(prefix_path).await;
        },
        ("rm", Some(clone_matches)) => {
            let path = match clone_matches.value_of("path") {
                Some(path) => path,
                None => "/"
            };
            println!("Removing file at path {}", path);
        },
        ("mkdir", Some(clone_matches)) => {
            let path = match clone_matches.value_of("path") {
                Some(path) => path,
                None => "/"
            };
            println!("Removing file at path {}", path);
            // let promise = S3Driver::list_files();
            // tokio::run(promise);
        },
        ("cat", Some(clone_matches)) => {
            let path = match clone_matches.value_of("path") {
                Some(path) => path,
                None => "/"
            };
            println!("Removing file at path {}", path);
        },
        _ => {
            unreachable!();
        }
    }
}

async fn list_files(prefix_path: &str) {
    let authentication_delegate = LocalAuthenticator::new();

    let builder = ListFilesCommandBuilder::new(
        OsString::from(prefix_path),
        &authentication_delegate 
    );
    let res = builder.run().await;

    let command = res.unwrap();
    //
    let handler = ListFilesCommandHandler::new(&command);
    let res = handler.run();
    match res {
        Ok(result) => println!("{:?}", result),
        Err(e) => println!("Error, {:?}", e)
    }
}
