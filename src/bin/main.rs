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

    // let mut parser = state::ParserWrapper::new(&WINAPI_ROOT_PATH.join("um/iphlpapi.h"))?;
    // let tokens = parser.parse()?;
    // debug!("tokens: {:?}", tokens);

    whitespace_fn();

    Ok(())
}

fn whitespace_fn() {
    use std::io::*;

    let mut source = String::new();
    match std::env::args().nth(1) {
        Some(filename) => {
            use std::fs::File;

            File::open(&filename)
                .expect(&format!("Can't open {}", &filename))
                .read_to_string(&mut source)
                .expect(&format!("Can't read contents of {}", &filename));
        }

        None => {
            stdin()
                .read_to_string(&mut source)
                .expect("Can't read stdin");
        }
    }

    if source.is_empty() {
        println!("Empty file");
        return;
    }

    debug!("Compiling whitespace source code");
    winapi_gen::compile(&source).expect("OH NO").interpret();
}