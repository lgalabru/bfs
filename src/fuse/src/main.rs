#![feature(async_await)]
#[macro_use] extern crate log;
extern crate env_logger;
use std::ffi::{OsStr};
use std::env;

mod bridge;
mod authenticator;
mod commands;

use bridge::Bridge;

fn main() {
    env_logger::init();
    let mountpoint = env::args_os().nth(1).unwrap();
    let options = ["-o", "ro", "-o", "fsname=hello"]
        .iter()
        .map(|o| o.as_ref())
        .collect::<Vec<&OsStr>>();

    let bridge = Bridge::new();
    fuse::mount(bridge, &mountpoint, &options).unwrap();
}