#![feature(async_await)]
use async_trait::async_trait;
use std::ffi::{CStr, CString};

use std::thread;

use std::io::{self, Read, Write, BufReader, BufRead};
use std::net::TcpListener;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;

use bfs_commands::{
    AuthenticationDelegate,
    AuthenticationResult,
    AuthenticationError,
    AuthenticationToken
};

#[derive(Debug)]
pub enum Error {
    Unknown,
}

pub struct Authenticator {
    app_private_key: Option<String>
}

impl Authenticator {
    pub fn new() -> Self {
        Self {
            app_private_key: None
        }
    }
}

fn handle_request<S>(mut stream: S)
    where S: Read + Write
{
    let request = parse_request(&mut stream);
    println!("{}", request);
    send_response(stream, &request, 200);
}

fn parse_request<R>(stream: R) -> String
    where R: Read
{
    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line).unwrap();
    line
}

fn send_response<W>(mut stream: W, res: &str, status: u32)
    where W: Write
{
    write!(&mut stream, "{}{}{}{}", "HTTP/1.1 ", status, " OK\n\n", res).unwrap();
}

fn start_authentication_response_server(mutexed_webview: &Arc<Mutex<WebView>>) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => handle_request(stream),
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}

impl Authenticator {

    async fn start_authorization_flow(&self) -> Result<AuthenticationToken, Error> {

        let manifest = "".to_string();
        let authorization_request_token =  "".to_string();

        // authorization_request_token
        let url = format!("https://browser.blockstack.org/{}", authorization_request_token);
        let content = web_view::Content::Url(url);
        // Open webview
        let webview = web_view::builder()
            .title("Authentication")
            .content(content)
            .size(800, 600)
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|webview, arg| {
                match arg {
                    "exit" => {
                        webview.terminate();
                    }
                    _ => {},
                };
                Ok(())
            })
            .build().unwrap();

        let mutexed_webview = Arc::new(Mutex::new(webview));

        let handler = thread::spawn(|| {
            start_authentication_response_server(&mutexed_webview)
        });
        
        webview.run();

        handler.join().unwrap();

        Err(Error::Unknown)
    }
}

#[async_trait]
impl AuthenticationDelegate for Authenticator {

    async fn get_authorization_token(&self) -> AuthenticationResult {
        // Handshake has already been perfomed
        if let Some(app_private_key) = &self.app_private_key {
            return Ok(AuthenticationToken::new(&app_private_key))
        }
        
        // Start authorization flow 
        let result = self.start_authorization_flow().await;
        let token = result.unwrap();
        println!("Authorization succeeded! {:?}", token);
        Ok(token)
    }
}

