// #[macro_use] extern crate failure_derive;

// mod header_parser;
// pub use header_parser::Parser;
// pub mod token;
// pub use token::Tokenizer;
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

pub mod ast;
pub mod eval;
pub mod lexer;

lalrpop_mod!(pub parser);

pub fn compile(input: &str) -> Result<ast::Program, String> {
    match parser::ProgramParser::new().parse(lexer::Lexer::new(input)) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}

#[test]
fn parse_simple() {
    let input = lexer::Lexer::new("\n\n\n");
    let program = parser::ProgramParser::new().parse(input).expect("Oh no");
    match (program.len(), program.first()) {
        (1, Some(&ast::Stmt::Exit)) => (),
        other => panic!("Well that didn't work: {:?}", other),
    }
}