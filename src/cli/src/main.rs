use clap::{
    Arg, 
    App,
    SubCommand, 
    AppSettings
};
use drivers_aws::s3::S3Driver;
use drivers::{
    driver::StorageDriver, 
    driver::ListFilesParams
};
use std::ffi::{OsString, OsStr};

use tokio;

use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
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
            println!("Listing files in {}", prefix_path);
            let params = ListFilesParams {
                prefix_path: OsString::from(prefix_path),
                page: None
            };
            let res = S3Driver::list_files(params);
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
