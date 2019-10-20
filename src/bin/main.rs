// #[macro_use] extern crate log;
// extern crate env_logger;

use lazy_static::lazy_static;
use log::info;
use std::path::Path;
use winapi_gen::HeaderFile;

static WINAPI_ROOT: &'static str = "../win-sdk-10.0.17134.0";

lazy_static!{
    pub static ref WINAPI_ROOT_PATH: &'static Path = &Path::new(WINAPI_ROOT);
}


#[cfg(not(test))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("Hello, world!");
    // let mut header = HeaderFile::new(&WINAPI_ROOT_PATH.join("um/iphlpapi.h"))?;
    // header.read_contents()?;
    // header.parse()?;
    match winapi_gen::header::HeaderParser::new().parse("/* This is also\n\t a comment, too */") {
        Ok(_) => {},
        Err(err) => panic!("{:?}", err),
    }
    Ok(())
}
