// #[macro_use] extern crate log;
// extern crate env_logger;

use lazy_static::lazy_static;
use log::info;
use std::path::Path;
use winapi_gen::{Parser, Tokenizer};

static WINAPI_ROOT: &'static str = "../win-sdk-10.0.17134.0";

lazy_static!{
    pub static ref WINAPI_ROOT_PATH: &'static Path = &Path::new(WINAPI_ROOT);
}


#[cfg(not(test))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    info!("Started winapi-gen");

    
    let mut parser = Parser::new(&WINAPI_ROOT_PATH.join("um/iphlpapi.h"))?;
    parser.open()?;
    for _ in 0..5 {
        let parsed_line = parser.read_line()?;
        let _tokens = Tokenizer::go(&parsed_line)?;
    }

    Ok(())
}
