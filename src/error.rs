use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Argument(String),
    Io(io::Error),
    Match(String),
    NotFound,
    UnFinished,
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Argument(err) => {
                write!(f, "{err}")
            }
            Error::Io(err) => {
                write!(f, "{err}")
            }
            Error::Match(err) => {
                write!(f, "{err}")
            }
            Error::NotFound => {
                write!(f, "not found")
            }
            Error::UnFinished => {
                write!(f, "unfinished")
            }
        }
    }
}
