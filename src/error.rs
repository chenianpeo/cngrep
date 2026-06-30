use std::fmt;
use std::io;

/// # Error Handle
///
/// include Args, IO and Internal error,
/// Args error is user input stage,
/// IO error is file read or result output stage,
/// Internal error is shouldn't appearance with software running,
#[derive(Debug)]
pub enum _Error {
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
}

impl std::error::Error for _Error {}

impl From<io::Error> for _Error {
    fn from(value: io::Error) -> Self {
        _Error::IO {
            source: value,
            context: None,
        }
    }
}

impl std::fmt::Display for _Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _Error::InvalidArg { r#type, context } => {
                write!(f, "invalid input {}: {}", r#type, context)
            }

            _Error::IO { source, context } => {
                if let Some(ctx) = context {
                    write!(f, "{}: {}", ctx, source)
                } else {
                    write!(f, "io error {}", source)
                }
            }

            _Error::Internal { context } => {
                write!(f, "internal error: {}", context)
            }
        }
    }
}
