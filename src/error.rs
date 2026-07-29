use std::fmt;
use std::io;

/// # Error Handle
///
/// include Args, IO and Internal error,
/// Args error is user input stage,
/// IO error is file read or result output stage,
/// Internal error is shouldn't appearance with software running,
#[derive(Debug)]
pub enum Error {
    InvalidArg {
        r#type: String,
        context: String,
    },

    IO {
        source: std::io::Error,
        context: Option<String>,
    },

    Internal {
        context: String,
    },

    Output {
        context: String,
    },
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO {
            source: value,
            context: None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidArg { r#type, context } => {
                write!(f, "invalid input {}: {}", r#type, context)
            }

            Error::IO { source, context } => {
                if let Some(ctx) = context {
                    write!(f, "{}: {}", ctx, source)
                } else {
                    write!(f, "io error {}", source)
                }
            }

            Error::Internal { context } => {
                write!(f, "internal error: {}", context)
            }

            Error::Output { context } => {
                write!(f, "{context}")
            }
        }
    }
}
