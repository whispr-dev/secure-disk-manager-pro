use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, SdmError>;

#[derive(Debug)]
pub enum SdmError {
    Io(std::io::Error),
    InvalidInput(String),
    InvalidFormat(String),
    Compression(String),
    Crypto(String),
    UnsupportedPlatform(&'static str),
    Blocked(&'static str),
    CommandFailed { program: String, status: Option<i32> },
}

impl Display for SdmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SdmError::Io(e) => write!(f, "I/O error: {e}"),
            SdmError::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            SdmError::InvalidFormat(msg) => write!(f, "invalid format: {msg}"),
            SdmError::Compression(msg) => write!(f, "compression error: {msg}"),
            SdmError::Crypto(msg) => write!(f, "crypto error: {msg}"),
            SdmError::UnsupportedPlatform(msg) => write!(f, "unsupported platform: {msg}"),
            SdmError::Blocked(msg) => write!(f, "blocked operation: {msg}"),
            SdmError::CommandFailed { program, status } => {
                write!(f, "command failed: {program} exited with {status:?}")
            }
        }
    }
}

impl Error for SdmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SdmError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SdmError {
    fn from(value: std::io::Error) -> Self {
        SdmError::Io(value)
    }
}
