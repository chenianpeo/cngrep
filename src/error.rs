use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CnError {
    Io(io::Error),
    Parse(String),
    InvalidInput(String),
}

impl fmt::Display for CnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CnError::Io(e) => write!(f, "io error: {}", e),
            CnError::Parse(msg) => write!(f, "parse error: {}", msg),
            CnError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

impl std::error::Error for CnError {}

impl From<io::Error> for CnError {
    fn from(err: io::Error) -> Self {
        CnError::Io(err)
    }
}
