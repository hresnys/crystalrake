use std::{num::{ParseIntError, ParseFloatError}, string::FromUtf16Error};

/// Enum to store the various types of errors that can cause tokenizing a JSON to fail.
#[derive(Debug)]
pub enum LexErrorKind {
    /// Contains an invalid char in a JSON.
    InvalidChar(char),
    /// Could not find a pair of quotation marks in string.
    NonQuotationMark,
    /// Could not find after minus sign.
    NotDigit,
    /// Fraction part didn't contain any digit.
    NonFracDigit,
    /// Found an invalid char after reverse solidus.
    NotEscapeChar,
    /// Found `"\uXXXX"`(X is a hex digit) from JSON, but `XXXX` could not parse to `u16`.
    ParseError(ParseIntError)
}

#[derive(Debug)]
pub struct JsonLexerError {
    pub(crate) kind: LexErrorKind,
}

impl JsonLexerError {
    pub fn kind(&self) -> &LexErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for JsonLexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            LexErrorKind::InvalidChar(c) => write!(f, "invalid charactor '{}' found from JSON", c),
            LexErrorKind::NonQuotationMark => write!(f, "cannot find a pair of quotation-mark from JSON string"),
            LexErrorKind::NonFracDigit => write!(f, "cannot find any digit after decimal-point"),
            LexErrorKind::NotDigit => write!(f, "cannot find any digit after minus sign"),
            LexErrorKind::NotEscapeChar => write!(f, "invalid charactor found after reverse solidus"),
            LexErrorKind::ParseError(_) => write!(f, "cannot parse hex digit string to u16"),
        }
    }
}

impl std::error::Error for JsonLexerError {}

/// Enums to store the various types of errors that can cause parsing a JSON to fail.
#[derive(Debug)]
pub enum ParseErrorKind {
    /// Contains an invalid token in a JSON.
    InvalidToken,
    /// Could not find any JSON value.
    NonValue,
    /// Found left curly brancket, but could not find right curly brancket.
    NonEndObject,
    /// Found left square brancket, but could not find right square brancket.
    NonEndArray,
    /// Could not find any JSON object name.
    NoObjectName,
    /// A possible error value when converting a String from a UTF-16 byte slice.
    FromUtf16Error(FromUtf16Error),
    ParseFloatError(ParseFloatError),
    LexError(JsonLexerError),
}

#[derive(Debug)]
pub struct JsonParseError {
    pub(crate) kind: ParseErrorKind,
}

impl JsonParseError {
    pub fn kind(&self) -> &ParseErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for JsonParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ParseErrorKind::InvalidToken => write!(f, "invalid token found in JSON"),
            ParseErrorKind::NonValue => write!(f, "expect some value, but cannot find any JSON value"),
            ParseErrorKind::NonEndObject => write!(f, "expect end-of-object '}}', but cannot find any right curly bracket"),
            ParseErrorKind::NonEndArray => write!(f, "expect end-of-array ']', but cannot find right square bracket"),
            ParseErrorKind::NoObjectName => write!(f, "cannot find any object name"),
            ParseErrorKind::FromUtf16Error(e) => e.fmt(f),
            ParseErrorKind::ParseFloatError(e) => e.fmt(f),
            ParseErrorKind::LexError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for JsonParseError {
    
}