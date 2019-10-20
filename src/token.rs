use failure::{format_err, Error};
// use std::fs;
// use std::io::{BufReader, BufRead};
// use std::path::{Path, PathBuf};
use log::{debug, error, info};
use super::parser::ParsedLine;

pub type TokenResult = Result<Vec<TokenSpan>, Error>;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    line: usize,
    column: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct TokenSpan {
    start: Location,
    end: Location,
    token: Token,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    SlashForward,
    SlashBackward,
    Asterisk,
    Plus,
    NewLine,
    // Identifer(String),
    // Comment(String),
}

pub struct Tokenizer {
    
}

impl Tokenizer {
    pub fn go(line: &ParsedLine) -> TokenResult  {
        let chars = line.buffer.chars();
        debug!("chars: [{:?}]", chars);
        let mut token_spans = Vec::with_capacity((chars.size_hint().1).expect("Shouldn't be passed a 0 length buffer of chars"));
        for (x, a_char) in chars.enumerate() {
            debug!("for:   x:{}  a_char:[{}]", x, a_char);
            let token = match a_char {
                '/'     => Token::SlashForward,
                '\\'    => Token::SlashBackward,
                '*'     => Token::Asterisk,
                '+'     => Token::Plus,
                '\n'     => Token::NewLine,
                _ => unimplemented!(),
            };
            let char_location = Location {line: line.line_num, column: x};
            let token_span = TokenSpan { start: char_location, end: char_location, token: token};
            token_spans.push(token_span);
        }

        debug!("token_spans: [{:?}]", token_spans);
        Ok(token_spans)
    }
}