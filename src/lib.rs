// #[macro_use] extern crate failure_derive;

mod parser;
pub use parser::Parser;
pub mod token;
// pub use token::Tokenizer;
pub mod state;


// #[test]
// fn header() {
//     assert!(header::HeaderParser::new().parse("/* This is a comment */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is also\n\t a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end /").is_err());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end, either *").is_err());
// }
