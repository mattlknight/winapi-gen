use matches::matches;
// use ::macros::match_byte;
use crate::cow_rc_str::CowRcStr;
// use std::borrow::{Borrow, Cow};
// use std::cmp;
// use std::fmt;
// use std::hash;
// use std::marker::PhantomData;
// use std::mem;
// use std::ops::Deref;
// use std::rc::Rc;
// use std::slice;
use std::str;
// use std::usize;
use self::Token::*;

/// One of the pieces the CSS input is broken into.
///
/// Some components use `Cow` in order to borrow from the original input string
/// and avoid allocating/copying when possible.
#[derive(PartialEq, Debug, Clone)]
pub enum Token<'a> {
    /// A [`<ident-token>`](https://drafts.csswg.org/css-syntax/#ident-token-diagram)
    Ident(CowRcStr<'a>),

    /// A [`<at-keyword-token>`](https://drafts.csswg.org/css-syntax/#at-keyword-token-diagram)
    ///
    /// The value does not include the `@` marker.
    AtKeyword(CowRcStr<'a>),

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "unrestricted"
    ///
    /// The value does not include the `#` marker.
    Hash(CowRcStr<'a>),

    /// A [`<hash-token>`](https://drafts.csswg.org/css-syntax/#hash-token-diagram) with the type flag set to "id"
    ///
    /// The value does not include the `#` marker.
    IDHash(CowRcStr<'a>), // Hash that is a valid ID selector.

    /// A [`<string-token>`](https://drafts.csswg.org/css-syntax/#string-token-diagram)
    ///
    /// The value does not include the quotes.
    QuotedString(CowRcStr<'a>),

    /// A [`<url-token>`](https://drafts.csswg.org/css-syntax/#url-token-diagram)
    ///
    /// The value does not include the `url(` `)` markers.  Note that `url( <string-token> )` is represented by a
    /// `Function` token.
    UnquotedUrl(CowRcStr<'a>),

    /// A `<delim-token>`
    Delim(char),

    /// A [`<number-token>`](https://drafts.csswg.org/css-syntax/#number-token-diagram)
    Number {
        /// Whether the number had a `+` or `-` sign.
        ///
        /// This is used is some cases like the <An+B> micro syntax. (See the `parse_nth` function.)
        has_sign: bool,

        /// The value as a float
        value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        int_value: Option<i32>,
    },

    /// A [`<percentage-token>`](https://drafts.csswg.org/css-syntax/#percentage-token-diagram)
    Percentage {
        /// Whether the number had a `+` or `-` sign.
        has_sign: bool,

        /// The value as a float, divided by 100 so that the nominal range is 0.0 to 1.0.
        unit_value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        /// It is **not** divided by 100.
        int_value: Option<i32>,
    },

    /// A [`<dimension-token>`](https://drafts.csswg.org/css-syntax/#dimension-token-diagram)
    Dimension {
        /// Whether the number had a `+` or `-` sign.
        ///
        /// This is used is some cases like the <An+B> micro syntax. (See the `parse_nth` function.)
        has_sign: bool,

        /// The value as a float
        value: f32,

        /// If the origin source did not include a fractional part, the value as an integer.
        int_value: Option<i32>,

        /// The unit, e.g. "px" in `12px`
        unit: CowRcStr<'a>,
    },

    /// A [`<whitespace-token>`](https://drafts.csswg.org/css-syntax/#whitespace-token-diagram)
    WhiteSpace(&'a str),

    /// A comment.
    ///
    /// The CSS Syntax spec does not generate tokens for comments,
    /// But we do, because we can (borrowed &str makes it cheap).
    ///
    /// The value does not include the `/*` `*/` markers.
    Comment(&'a str),

    /// A `:` `<colon-token>`
    Colon, // :

    /// A `;` `<semicolon-token>`
    Semicolon, // ;

    /// A `,` `<comma-token>`
    Comma, // ,

    /// A `~=` [`<include-match-token>`](https://drafts.csswg.org/css-syntax/#include-match-token-diagram)
    IncludeMatch,

    /// A `|=` [`<dash-match-token>`](https://drafts.csswg.org/css-syntax/#dash-match-token-diagram)
    DashMatch,

    /// A `^=` [`<prefix-match-token>`](https://drafts.csswg.org/css-syntax/#prefix-match-token-diagram)
    PrefixMatch,

    /// A `$=` [`<suffix-match-token>`](https://drafts.csswg.org/css-syntax/#suffix-match-token-diagram)
    SuffixMatch,

    /// A `*=` [`<substring-match-token>`](https://drafts.csswg.org/css-syntax/#substring-match-token-diagram)
    SubstringMatch,

    /// A `<!--` [`<CDO-token>`](https://drafts.csswg.org/css-syntax/#CDO-token-diagram)
    CDO,

    /// A `-->` [`<CDC-token>`](https://drafts.csswg.org/css-syntax/#CDC-token-diagram)
    CDC,

    /// A [`<function-token>`](https://drafts.csswg.org/css-syntax/#function-token-diagram)
    ///
    /// The value (name) does not include the `(` marker.
    Function(CowRcStr<'a>),

    /// A `<(-token>`
    ParenthesisBlock,

    /// A `<[-token>`
    SquareBracketBlock,

    /// A `<{-token>`
    CurlyBracketBlock,

    /// A `<bad-url-token>`
    ///
    /// This token always indicates a parse error.
    BadUrl(CowRcStr<'a>),

    /// A `<bad-string-token>`
    ///
    /// This token always indicates a parse error.
    BadString(CowRcStr<'a>),

    /// A `<)-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseParenthesis,

    /// A `<]-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseSquareBracket,

    /// A `<}-token>`
    ///
    /// When obtained from one of the `Parser::next*` methods,
    /// this token is always unmatched and indicates a parse error.
    CloseCurlyBracket,
}

impl<'a> Token<'a> {
    /// Return whether this token represents a parse error.
    ///
    /// `BadUrl` and `BadString` are tokenizer-level parse errors.
    ///
    /// `CloseParenthesis`, `CloseSquareBracket`, and `CloseCurlyBracket` are *unmatched*
    /// and therefore parse errors when returned by one of the `Parser::next*` methods.
    pub fn is_parse_error(&self) -> bool {
        matches!(
            *self,
            BadUrl(_) | BadString(_) | CloseParenthesis | CloseSquareBracket | CloseCurlyBracket
        )
    }
}