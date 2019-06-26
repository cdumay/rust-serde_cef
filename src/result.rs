use std::error::Error;
use std::fmt;

/// Enum to store errors
///
/// Mainly based on parsing errors, this enum implement the trait `std::error::Error`.
#[derive(Debug)]
pub enum CefError {
    UnknownError(String),
    IoError(std::io::Error),
    RegexError(regex::Error),
    NoneError,
    ParseIntError(std::num::ParseIntError),
    InvalidDateFormat(chrono::ParseError),
    ParseFloatError(std::num::ParseFloatError),
    ValidationError(String),
    Utf8Error(std::str::Utf8Error),
    DeserializerError(serde_value::DeserializerError),
    SerializerError(serde_value::SerializerError),
    FromUtf8Error(std::string::FromUtf8Error),
}

/// Alias for a `Result` with the error type `CefError`.
pub type CefResult<T> = Result<T, CefError>;

impl fmt::Display for CefError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CefError::UnknownError(text) => f.write_str(text),
            CefError::IoError(err) => err.fmt(f),
            CefError::RegexError(err) => err.fmt(f),
            CefError::NoneError => f.write_str("Empty option!"),
            CefError::ParseIntError(err) => err.fmt(f),
            CefError::InvalidDateFormat(err) => err.fmt(f),
            CefError::ParseFloatError(err) => err.fmt(f),
            CefError::ValidationError(text) => f.write_str(text),
            CefError::Utf8Error(err) => err.fmt(f),
            CefError::DeserializerError(err) => err.fmt(f),
            CefError::SerializerError(err) => err.fmt(f),
            CefError::FromUtf8Error(err) => err.fmt(f),
        }
    }
}

impl Error for CefError {}

impl From<std::string::FromUtf8Error> for CefError {
    fn from(err: std::string::FromUtf8Error) -> CefError {
        CefError::FromUtf8Error(err)
    }
}

impl From<serde_value::SerializerError> for CefError {
    fn from(err: serde_value::SerializerError) -> CefError {
        CefError::SerializerError(err)
    }
}

impl From<serde_value::DeserializerError> for CefError {
    fn from(err: serde_value::DeserializerError) -> CefError {
        CefError::DeserializerError(err)
    }
}

impl From<std::str::Utf8Error> for CefError {
    fn from(err: std::str::Utf8Error) -> CefError {
        CefError::Utf8Error(err)
    }
}

impl From<chrono::ParseError> for CefError {
    fn from(err: chrono::ParseError) -> CefError {
        CefError::InvalidDateFormat(err)
    }
}

impl From<std::num::ParseFloatError> for CefError {
    fn from(err: std::num::ParseFloatError) -> CefError {
        CefError::ParseFloatError(err)
    }
}

impl From<std::num::ParseIntError> for CefError {
    fn from(err: std::num::ParseIntError) -> CefError {
        CefError::ParseIntError(err)
    }
}

impl From<std::option::NoneError> for CefError {
    fn from(_err: std::option::NoneError) -> CefError {
        CefError::NoneError
    }
}

impl From<regex::Error> for CefError {
    fn from(err: regex::Error) -> CefError {
        CefError::RegexError(err)
    }
}

impl From<&str> for CefError {
    fn from(data: &str) -> CefError {
        CefError::UnknownError(data.into())
    }
}

impl From<std::io::Error> for CefError {
    fn from(err: std::io::Error) -> CefError {
        CefError::IoError(err)
    }
}
