#![allow(unused_imports)]

// #[macro_use] extern crate log;
// extern crate env_logger;

use lazy_static::lazy_static;
use log::{info, debug};
use std::path::Path;
// use winapi_gen::{Parser, state};

static WINAPI_ROOT: &'static str = "../win-sdk-10.0.17134.0";

lazy_static!{
    pub static ref WINAPI_ROOT_PATH: &'static Path = &Path::new(WINAPI_ROOT);
}


#[cfg(not(test))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Started winapi-gen");

    let header_file_path = &WINAPI_ROOT_PATH.join("um/iphlpapi.h");
    let _file = winapi_gen::file::read_complete_file(header_file_path)?;

    // let mut parser = state::ParserWrapper::new())?;
    // let tokens = parser.parse()?;
    // debug!("tokens: {:?}", tokens);
    // println!("{}", file);
    // winapi_gen::compile(&file).expect("OH NO").interpret();

    Ok(())
}
