use std::ffi::OsString;

use clap::{
    Arg, 
    App,
    SubCommand
};

use commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
    AuthenticationDelegate
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

impl AuthenticationDelegate for LocalAuthenticator {
    fn get_authorization_token(&self) -> String {
        println!("Authentication in progress...");
        "0x1234567890ABCDEF".to_string()
    }
}

fn main() {
    let authentication_delegate = LocalAuthenticator::new();

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
            // 
            let builder = ListFilesCommandBuilder::new(
                OsString::from(prefix_path),
                &authentication_delegate 
            );
            let command = builder.run();
            //
            let handler = ListFilesCommandHandler::new(&command);
            let res = handler.run();
            match res {
                Ok(result) => println!("{:?}", result),
                Err(e) => println!("Error, {:?}", e)
            }
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
