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

#[cfg(test)]
mod test_error {
    use super::*;

    #[test]
    fn test_invalid_pattern() {
        let err = Error::InvalidPattern {
            pattern: "[a-z".to_string(),
            reason: "missing closing bracket".to_string(),
        };
        match err {
            Error::InvalidPattern { pattern, reason } => {
                assert_eq!(pattern, "[a-z".to_string());
                assert_eq!(reason, "missing closing bracket".to_string())
            }
            _ => panic!("expected InvalidPattern error"),
        }
    }

    #[test]
    fn test_io() {
        let err = Error::Io {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
            context: Some("read file report error".to_string()),
        };
        match err {
            Error::Io { source, context } => {
                assert_eq!(source.kind(), std::io::ErrorKind::NotFound);
                assert_eq!(source.to_string(), "file not found");
                assert_eq!(context, Some("read file report error".to_string()));
            }
            _ => panic!("expected Io error"),
        }
    }

    #[test]
    fn test_internal() {
        let err = Error::Internal {
            message: "unexpected empty state".to_string(),
        };
        match err {
            Error::Internal { message } => {
                assert_eq!(message, "unexpected empty state");
            }
            _ => panic!("expected Internal error"),
        }
    }
}
