// #[macro_use] extern crate failure_derive;

mod parser;
pub use parser::Parser;
mod token;
pub use token::Tokenizer;


// #[test]
// fn header() {
//     assert!(header::HeaderParser::new().parse("/* This is a comment */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This is also\n\t a comment, too */").is_ok());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end /").is_err());
//     assert!(header::HeaderParser::new().parse("/* This comment doesn't end, either *").is_err());
// }
