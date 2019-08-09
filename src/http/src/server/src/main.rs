#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::ffi::OsString;

use commands::{
    list_files::{ListFilesCommandBuilder, ListFilesCommandHandler},
    AuthenticationDelegate
};

struct WebAuthenticator;

impl WebAuthenticator {
    pub fn new() -> WebAuthenticator {
        WebAuthenticator {}
    }
}

impl AuthenticationDelegate for WebAuthenticator {
    fn get_authorization_token(&self) -> String {
        println!("Authentication in progress...");
        "0x1234567890ABCDEF".to_string()
    }
}

#[get("/")]
fn index() -> String {

    let authentication_delegate = WebAuthenticator::new();

    let prefix_path = "/";
    //
    let builder = ListFilesCommandBuilder::new(
        OsString::from(prefix_path),
        &authentication_delegate 
    );
    let command = builder.run();
    //
    let handler = ListFilesCommandHandler::new(&command);
    let wrapped_res = handler.run();

    let res = match wrapped_res {
        Ok(result) => format!("{:?}", result).clone(),
        Err(e) => format!("Error, {:?}", e).clone()
    };

    res
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}