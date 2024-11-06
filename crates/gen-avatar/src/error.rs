use std::io;
use std::num::ParseIntError;

/// Custom Error type for Avatar
#[derive(Debug)]
pub enum Error {
    /// Invalid hex string
    InvalidHexFormat { actual: String, expected: String },
    /// Parse error
    Parse(ParseIntError),
    /// IO read/write error
    IO(io::Error),
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Parse(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IO(error)
    }
}
