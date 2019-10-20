#[macro_use] extern crate failure_derive;
use lalrpop_util::lalrpop_mod;

pub mod ast;
mod header_file;
pub use header_file::HeaderFile;

lalrpop_mod!(pub header);

#[test]
fn header() {
    assert!(header::HeaderParser::new().parse("/* This is a comment */").is_ok());
    assert!(header::HeaderParser::new().parse("/* This is a comment, too */").is_ok());
    assert!(header::HeaderParser::new().parse("/* This is also\n\t a comment, too */").is_ok());
    assert!(header::HeaderParser::new().parse("/* This comment doesn't end /").is_err());
    assert!(header::HeaderParser::new().parse("/* This comment doesn't end, either *").is_err());
}
//
// #[test]
// fn calculator6() {
//     let mut errors = Vec::new();
//
//     let expr = calculator6::ExprsParser::new()
//         .parse(&mut errors, "22 * + 3")
//         .unwrap();
//     assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");
//
//     let expr = calculator6::ExprsParser::new()
//         .parse(&mut errors, "22 * 44 + 66, *3")
//         .unwrap();
//     assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");
//
//     let expr = calculator6::ExprsParser::new()
//         .parse(&mut errors, "*")
//         .unwrap();
//     assert_eq!(&format!("{:?}", expr), "[(error * error)]");
//
//     assert_eq!(errors.len(), 4);
// }
