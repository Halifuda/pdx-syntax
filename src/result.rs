//! Warpper for parser result.
//!
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Error type.
/// - ParseError: parsing failed. This is formated to [`String`].
/// - IoError: IO error.
#[derive(Debug)]
pub enum Error {
    ParseError(String),
    IoError(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
            Error::ParseError(e) => write!(f, "ParseError: {}", e),
            Error::IoError(e) => write!(f, "IoError: {}", e),
        }
    }
}

impl std::error::Error for Error {}
