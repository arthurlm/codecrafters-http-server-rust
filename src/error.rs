use std::{io, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Clone, Error)]
pub enum HttpServerError {
    #[error("I/O error: {0}")]
    Io(String),

    #[error("missing header: {0}")]
    MissingHeader(&'static str),

    #[error("invalid header value: {0}")]
    InvalidHeaderValue(String),
}

impl From<io::Error> for HttpServerError {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<ParseIntError> for HttpServerError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidHeaderValue(err.to_string())
    }
}
