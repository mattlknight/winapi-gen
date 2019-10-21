use log::debug;
use super::token::{Location, Token, TokenSpan};
use std::{thread, time};
use std::fmt;
use std::path::Path;
use super::header_parser::Parser;
use failure::Error;

// Goals for State Machine
// 1. Takes ()
// 2. No Allocations
// 3. Returns (Option<PreviousChar>, CharBuffer) or TokenSpan


// Getting started
// start with a line of text ending in a newline char
// turn the line of text into an iterator of chars
// start the state machine with the iterator of chars
// look at the first char, decide which token matches that char
// if the token is special, then look at the following char
// determine the correct token for pair of chars
// move to the next state based on pair of chars
// if the end state is reached, return the token span

// state machine needs an internal context to hold the thing that it is building up

pub type ParseResult<T> = Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub struct CharSpan<'span> {
    location: Location,
    this_char: &'span char,
}

impl<'span> fmt::Display for CharSpan<'span> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]{}", self.this_char, self.location)
    }
}

pub struct StateMachine<S> {
    pub state: S,
    pub token_spans: Vec<TokenSpan>,
}

impl<'state> StateMachine<Start<'state>> {
    pub fn start(line_num: usize, char_buffer: &'state[char]) -> Self {
        debug!("StateMachine::new() line_num: {} buff_len: {}", line_num, char_buffer.len());
        StateMachine {
            state: Start::new(line_num, &char_buffer),
            token_spans: Vec::new(),
        }
    }
}

impl StateMachine<End> {
    pub fn end() -> Self {
        debug!("StateMachine::end()");
        StateMachine {
            state: End {},
            token_spans: Vec::new(),
        }
    }
}

pub struct Start<'state> {
    pub char_buffer: &'state[char],
    pub this_char_span: CharSpan<'state>,
    pub next_char_span: Option<CharSpan<'state>>,
}

impl<'state> Start<'state> {
    pub fn new(line_num: usize, char_buffer: &'state [char]) -> Self {
        debug!("Start::new() line_num: {} buff_len: {}", line_num, char_buffer.len());
        let char_location = Location { line: line_num, column: 1 };
        let (new_char, char_buffer) = char_buffer.split_first().expect("First char in StateStart must be a valid char. Otherwise, what's the point even?!");
        let next_char = char_buffer.first();

        debug!("Start::new() char_location: {} new_char: {:?} next_char: {:?} buff_len: {}", char_location, new_char, next_char, char_buffer.len());

        Start {
            char_buffer: char_buffer,
            this_char_span: CharSpan { location: char_location, this_char: new_char},
            next_char_span: match next_char{
                None => None,
                Some(ref char_ref) => Some(CharSpan { location: char_location.add_column(), this_char: char_ref})
            },
        }
    }
}

pub struct Comment<'state> {
    pub char_buffer: &'state[char],
    pub comment: Vec<CharSpan<'state>>,
    pub this_char_span: CharSpan<'state>,
    pub next_char_span: Option<CharSpan<'state>>,
}

impl<'state> From<Comment<'state>> for TokenSpan {
    fn from(val: Comment) -> TokenSpan {
        let mut comment = String::new();
        for a_char in &val.comment {
            comment.push(*a_char.this_char);
        }
        TokenSpan {
            start: val.comment.first().expect("FIXME: 1").location,
            end: val.comment.last().expect("FIXME: 2").location,
            token: Token::Comment(comment),
        }
    }
}

impl<'state> From<StateMachine<Start<'state>>> for StateMachine<Comment<'state>> {
    fn from(val: StateMachine<Start<'state>>) -> StateMachine<Comment<'state>> {
        debug!("StateMachine<Comment>::from(StateMachine<Start>)");

        let char_location = val.state.this_char_span.location;
        let (this_char, char_buffer) = val.state.char_buffer.split_first().expect("First char in StateStart must be a valid char. Otherwise, what's the point even?!");
        let next_char = char_buffer.first();

        debug!("StateMachine<MultiLineComment>::from(StateMachine<Start>) char_location: {} this_char: {:?} next_char: {:?} buff_len: {}", char_location, this_char, next_char, char_buffer.len());

        StateMachine {
            state: Comment {
                char_buffer: val.state.char_buffer,
                comment: vec![val.state.this_char_span],
                this_char_span: CharSpan { location: char_location, this_char},
                next_char_span: match next_char{
                    None => None,
                    Some(ref char_ref) => Some(CharSpan { location: char_location.add_column(), this_char: char_ref})
                },
            },
            token_spans: val.token_spans,
        }
    }
}

impl<'state> From<StateMachine<Start<'state>>> for StateMachine<MultiLineComment<'state>> {
    fn from(val: StateMachine<Start<'state>>) -> StateMachine<MultiLineComment<'state>> {
        debug!("StateMachine<MultiLineComment>::from(StateMachine<Start>)");
        
        let next_char_location = val.state.this_char_span.location.add_column();
        let (new_char, char_buffer) = val.state.char_buffer.split_first().expect("First char in StateStart must be a valid char. Otherwise, what's the point even?!");
        let next_char = char_buffer.first();

        debug!("StateMachine<MultiLineComment>::from(StateMachine<Start>) char_location: {} this_char: {:?} next_char: {:?} buff_len: {}", next_char_location, new_char, next_char, char_buffer.len());

        let comment = vec![val.state.this_char_span];
        debug!("Comment: [{:?}]", comment);

        StateMachine {
            state: MultiLineComment {
                char_buffer: char_buffer,
                comment: comment,
                this_char_span: CharSpan { location: next_char_location, this_char: new_char},
                next_char_span: match next_char {
                    None => None,
                    Some(ref char_ref) => Some(CharSpan { location: next_char_location.add_column(), this_char: char_ref})
                },
            },
            token_spans: val.token_spans,
        }
    }
}

impl<'state> StateMachine<MultiLineComment<'state>> {
    fn append(val: StateMachine<MultiLineComment<'state>>) -> StateMachine<MultiLineComment<'state>> {
        debug!("StateMachine<MultiLineComment>::append()");
        
        let mut next_char_location = val.state.this_char_span.location.add_column();
        let (new_char, char_buffer) = val.state.char_buffer.split_first().expect("First char in StateStart must be a valid char. Otherwise, what's the point even?!");
        let next_char = char_buffer.first();

        debug!("StateMachine<MultiLineComment>::append() char_location: {} this_char: {:?} next_char: {:?} buff_len: {}", next_char_location, new_char, next_char, char_buffer.len());
        
        let mut comment = val.state.comment;
        comment.push(val.state.this_char_span);
        debug!("Comment: [{:?}]", comment);

        StateMachine {
            state: MultiLineComment {
                char_buffer: char_buffer,
                comment: comment,
                this_char_span: CharSpan { location: next_char_location, this_char: new_char},
                next_char_span: match next_char {
                    None => None,
                    Some(ref char_ref) => Some(CharSpan { location: next_char_location.add_column(), this_char: char_ref})
                },
            },
            token_spans: val.token_spans,
        }
    }
}

impl<'state> StateMachine<MultiLineComment<'state>> {
    fn finish(mut val: StateMachine<MultiLineComment<'state>>) -> StateMachine<Start> {
        debug!("StateMachine<MultiLineComment>::finish()");

        let next_char_location = val.state.this_char_span.location.add_column();
        let (new_char, char_buffer) = val.state.char_buffer.split_first().expect("First char in StateStart must be a valid char. Otherwise, what's the point even?!");
        let next_char = char_buffer.first();

        debug!("StateMachine<MultiLineComment>::append() char_location: {} this_char: {:?} next_char: {:?} buff_len: {}", next_char_location, new_char, next_char, char_buffer.len());

        val.state.comment.push(val.state.this_char_span);
        debug!("Comment: [{:?}]", val.state.comment);
        val.token_spans.push(TokenSpan::from(val.state));

        StateMachine {
            state: Start {
                char_buffer: char_buffer,
                this_char_span: CharSpan { location: next_char_location, this_char: new_char},
                next_char_span: match next_char {
                    None => None,
                    Some(ref char_ref) => Some(CharSpan { location: next_char_location.add_column(), this_char: char_ref})
                },
            },
            token_spans: val.token_spans,
        }
    }
}

pub struct MultiLineComment<'state> {
    pub char_buffer: &'state[char],
    pub comment: Vec<CharSpan<'state>>,
    pub this_char_span: CharSpan<'state>,
    pub next_char_span: Option<CharSpan<'state>>,
}

impl<'state> From<MultiLineComment<'state>> for TokenSpan {
    fn from(val: MultiLineComment) -> TokenSpan {
        let mut comment = String::new();
        for a_char in &val.comment {
            comment.push(*a_char.this_char);
        }
        TokenSpan {
            start: val.comment.first().expect("FIXME: 1").location,
            end: val.comment.last().expect("FIXME: 2").location,
            token: Token::Comment(comment),
        }
    }
}

pub struct StateIdentifier<'state> {
    pub char_buffer: &'state[char],
}

pub struct StateConstant<'state> {
    pub char_buffer: &'state[char],
}

pub struct End { }

impl<'machine, 'state> From<StateMachine<Comment<'state>>> for StateMachine<End> {
    fn from(val: StateMachine<Comment<'state>>) -> StateMachine<End> {
        debug!("StateMachine<End>::from(StateMachine<Comment>)");
        StateMachine {
            state: End { },
            token_spans: val.token_spans,
        }
    }
}

pub enum StateMachineWrapper<'state> {
    New,
    Start(StateMachine<Start<'state>>),
    Comment(StateMachine<Comment<'state>>),
    MultiLineComment(StateMachine<MultiLineComment<'state>>),
    End(StateMachine<End>),
}

impl<'machine, 'state> StateMachineWrapper<'state> {
    pub fn step(mut self) -> Self {
        debug!("StateMachineWrapper::step()");
        thread::sleep(time::Duration::from_secs(1));
        self = match self {
            StateMachineWrapper::New => StateMachineWrapper::New,
            StateMachineWrapper::Start(val) => {
                let this_char = val.state.this_char_span.this_char;
                let next_char = val.state.next_char_span;

                debug!("Currently in State: StateMachineWrapper::Start");
                debug!("this_char: {:?} next_char: {:?}", this_char, next_char);
                match this_char {
                    '/'     => {
                        match next_char {
                            Some(ref char_span) => match char_span.this_char {
                                '/' =>  StateMachineWrapper::Comment(val.into()),
                                '*' => StateMachineWrapper::MultiLineComment(val.into()),
                                _ => unimplemented!(),
                            }
                            None => unimplemented!(),
                        }
                    },
                    // '\n' => StateMachineWrapper::End(StateMachine::end()),
                    _ => unimplemented!(),
                }
            },
            StateMachineWrapper::Comment(_val) => unimplemented!(),
            StateMachineWrapper::MultiLineComment(val) => {
                let this_char = val.state.this_char_span.this_char;
                let mut next_char = val.state.next_char_span;

                debug!("Currently in State: StateMachineWrapper::MultiLineComment");
                debug!("this_char: {:?}", this_char);
                // if next_char.is_none() {
                //     return StateMachineWrapper::MultiLineComment(val);
                // }
                match this_char {
                    '*'     => {
                        debug!("Matched *");
                        match next_char {
                            Some(ref mut char_span) => {
                                if char_span.this_char == &'\n' {
                                    debug!("next_char.this_char == \\n");
                                    char_span.location = char_span.location.add_line();
                                    debug!("next_char.unwrap().location == {}", char_span.location);
                                }
                                debug!("next_char: {:?}", char_span);

                                match char_span.this_char {
                                '/' =>  {
                                    debug!("Matched /");
                                    StateMachineWrapper::Start(StateMachine::<MultiLineComment>::finish(val))
                                },
                                _ => StateMachineWrapper::MultiLineComment(StateMachine::<MultiLineComment>::append(val))
                            }}
                            None => {
                                debug!("Matched None");
                                unimplemented!();
                            },
                        }
                    },
                    '\n' => {
                        debug!("Matched \\n");
                        StateMachineWrapper::End(StateMachine::end())
                        },
                    _ => {
                        debug!("Matched _");
                        StateMachineWrapper::MultiLineComment(StateMachine::<MultiLineComment>::append(val))
                    },
                }
            },
            StateMachineWrapper::End(_val) => unimplemented!(),
        };
        self
    }
}

pub struct ParserFactory<'factory> {
    pub machine_wrapper: StateMachineWrapper<'factory>,
}

impl<'factory> ParserFactory<'factory> {
    pub fn new() -> ParseResult<Self> {
        debug!("ParserFactory::new()");

        Ok(ParserFactory {
            machine_wrapper: StateMachineWrapper::New,
        })
    }

    pub fn parse<'parse>(mut self, mut line_num: usize, char_buffer: &'factory [char], parser: &'factory mut Parser) -> ParseResult<Vec<TokenSpan>> {
        debug!("ParserFactory::parse()");

        self.machine_wrapper = StateMachineWrapper::Start(StateMachine::start(line_num, &char_buffer));

        loop {
            debug!("ParserFactory::parse() loop");
            self.machine_wrapper = self.machine_wrapper.step();
            match self.machine_wrapper {
                StateMachineWrapper::End(val) => return Ok(val.token_spans),
                _ => {},
            }
        }
    }
}

pub struct ParserWrapper {
    parser: Parser,
}

impl ParserWrapper {
    pub fn new<T: AsRef<Path>>(path: T) -> ParseResult<Self> {
        Ok(ParserWrapper {
            parser: Parser::new(path)?,
        })
    }

    pub fn parse(&mut self) -> ParseResult<Vec<TokenSpan>> {
        self.parser.open()?;

        let mut tokens = Vec::new();

        let parsed_line = self.parser.read_string()?;
        let chars: Vec<char> = parsed_line.buffer.chars().collect(); // FIXME: Thats an allocaiton right there!
        let factory = ParserFactory::new()?;
        tokens.append(&mut factory.parse(parsed_line.line_num, &chars, &mut self.parser)?);
        Ok(tokens)
    }
}