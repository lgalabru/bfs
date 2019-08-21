#![feature(async_await)]

use async_trait::async_trait;
use tokio;

use std::ffi::OsString;

use commands::{
    list_files::{
        ListFilesCommandBuilder, 
        ListFilesCommandHandler
    },
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

struct HTTPHeaderAuthenticator;

impl HTTPHeaderAuthenticator {
    pub fn new() -> LocalAuthenticator {
        LocalAuthenticator {}
    }
}

#[async_trait]
impl AuthenticationDelegate for HTTPHeaderAuthenticator {
    async fn get_authorization_token(&self) -> AuthenticationResult {
        println!("Authentication in progress...");
        Ok(AuthenticationToken::new("0x1234567890ABCDEF"))
    }
}

async fn hello(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let authentication_delegate = HeaderAuthenticator::new();

    let path = "/";
    //
    let builder = ListFilesCommandBuilder::new(
        OsString::from(path),
        &authentication_delegate 
    );
    let res = builder.run().await;

    let command = res.unwrap();
    //
    let handler = ListFilesCommandHandler::new(&command);

    let wrapped_res = handler.run();

    let res = match wrapped_res {
        Ok(result) => format!("{:?}", result).clone(),
        Err(e) => format!("Error, {:?}", e).clone()
    };

    Ok(Response::new(Body::from(res)))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(make_service_fn(|_| {
            // This is the `Service` that will handle the connection.
            // `service_fn` is a helper to convert a function that
            // returns a Response into a `Service`.
            async {
                Ok::<_, hyper::Error>(service_fn(hello))
            }
        }));

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}