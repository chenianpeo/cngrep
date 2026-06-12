use std::fmt;
use std::io;

// define custom error enum
#[derive(Debug)]
pub enum CnError {
    Io(io::Error),
    Parse(String),
    Custom(String),
}

impl fmt::Display for CnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CnError::Io(e) => write!(f, "io error: {}", e),
            CnError::Parse(msg) => write!(f, "parse error: {}", msg),
            CnError::Custom(msg) => write!(f, "custom return: {}", msg),
        }
    }
}

impl std::error::Error for CnError {}

impl From<io::Error> for CnError {
    fn from(err: io::Error) -> Self {
        CnError::Io(err)
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidPattern {
        pattern: String,
        reason: String,
    },
    Io {
        source: std::io::Error,
        context: Option<String>,
    },
    Internal {
        message: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidPattern { pattern, reason } => {
                write!(f, "invalid pattern `{}`: {}", pattern, reason)
            }
            Error::Io { source, context } => {
                if let Some(ctx) = context {
                    write!(f, "{}: {}", ctx, source)
                } else {
                    write!(f, "IO error: {}", source)
                }
            }
            Error::Internal { message } => {
                write!(f, "Internal error: {}", message)
            }
        }
    }
}
