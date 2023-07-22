use std::io;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Error {
    InvalidCharacter,
    EmptyCurlyBraces,
    OrphanCurlyBrace,
    OrphanSquareBrace,
    MaxDepthReached,
    InvalidQuote,
    InvalidComma,
    InvalidColon,
    InvalidState,
    IncompleteElement,
    InvalidKey(String),
    NoStartCurlyBrace,
    DuplicateKey(String)
}

impl From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCharacter => f.write_str("invalid character"),
            Error::EmptyCurlyBraces => f.write_str("empty curly braces"),
            Error::OrphanCurlyBrace => f.write_str("orphan curly brace"),
            Error::OrphanSquareBrace => f.write_str("orphan square brace"),
            Error::MaxDepthReached => f.write_str("max depth reached"),
            Error::InvalidQuote => f.write_str("invalid quote"),
            Error::InvalidComma => f.write_str("invalid comma"),
            Error::InvalidColon => f.write_str("invalid colon"),
            Error::InvalidState => f.write_str("invalid state"),
            Error::IncompleteElement => f.write_str("incomplete element"),
            Error::InvalidKey(key) => f.write_fmt(format_args!("Invalid Key starting with `{}`", key)), 
            Error::NoStartCurlyBrace => f.write_str("Missing the start Curly Brace"),
            Error::DuplicateKey(key) => f.write_fmt(format_args!("Duplicate Key `{}`", key)), 
        }
    }
}