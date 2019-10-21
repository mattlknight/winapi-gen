// #[macro_use] extern crate failure_derive;

// pub mod state;

// #[test]
// fn header() {
//     assert!(header::HeaderParser::new().parse("/* This is a comment */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is also\n\t a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end /").is_err());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end, either *").is_err());
// }

#[macro_use]
extern crate lalrpop_util;

#[macro_use]
mod macros;

pub mod ast;
pub mod cow_rc_str;
pub mod eval;
pub mod lexer;
pub mod file;
pub mod parser;
pub mod token;
#[cfg(feature = "dummy_match_byte")]
mod tokenizer;

#[cfg(not(feature = "dummy_match_byte"))]
mod tokenizer {
    include!(concat!(env!("OUT_DIR"), "/tokenizer.rs"));
}

lalrpop_mod!(pub ws_parser);

pub fn compile(input: &str) -> Result<ast::Program, String> {
    match ws_parser::ProgramParser::new().parse(lexer::Lexer::new(input)) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn parse_simple() {
    let input = lexer::Lexer::new("\n\n\n");
    let program = ws_parser::ProgramParser::new().parse(input).expect("Oh no");
    match (program.len(), program.first()) {
        (1, Some(&ast::Stmt::Exit)) => (),
        other => panic!("Well that didn't work: {:?}", other),
    }
}