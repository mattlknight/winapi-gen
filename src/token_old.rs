use failure::Error;
// use failure::format_err;
// use std::fs;
// use std::io::{BufReader, BufRead};
// use std::path::{Path, PathBuf};
// use log::{debug, error, info};
use log::debug;
// use super::parser::ParsedLine;
use std::fmt;

pub type TokenResult = Result<Vec<TokenSpan>, Error>;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn add_line(&self) -> Self {
        debug!("Location: {} add_line()", self);
        let new_location = Location {
            line: self.line + 1,
            column: 1,
        };
        debug!("new_location: {}", new_location);
        new_location
    }
    pub fn add_column(&self) -> Self {
        Location {
            line: self.line,
            column: self.column + 1,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

#[derive(Debug)]
pub struct TokenSpan {
    pub start: Location,
    pub end: Location,
    pub token: Token,
}

#[derive(Debug)]
pub enum Token {
    SlashForward,
    SlashBackward,
    Asterisk,
    Plus,
    NewLine,
    Identifer(String),
    Comment(String),
}

// pub struct Tokenizer {
    
// }

// impl Tokenizer {
//     pub fn go(line: &ParsedLine) -> TokenResult  {
//         let chars = line.buffer.chars();
//         debug!("chars: [{:?}]", chars);
//         let mut token_spans = Vec::with_capacity((chars.size_hint().1).expect("Shouldn't be passed a 0 length buffer of chars"));
//         for (x, a_char) in chars.enumerate() {
//             debug!("for:   x:{}  a_char:[{}]", x, a_char);
//             let token = match a_char {
//                 '/'     => Token::SlashForward,
//                 '\\'    => Token::SlashBackward,
//                 '*'     => Token::Asterisk,
//                 '+'     => Token::Plus,
//                 '\n'     => Token::NewLine,
//                 _ => unimplemented!(),
//             };
//             let char_location = Location {line: line.line_num, column: x};
//             let token_span = TokenSpan { start: char_location, end: char_location, token: token};
//             token_spans.push(token_span);
//         }

//         debug!("token_spans: [{:?}]", token_spans);
//         Ok(token_spans)
//     }
// }