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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn err_args() {
        let err = Error::Argument("invalid arguments".into());
        assert_eq!(err.to_string(), "invalid arguments");
    }

    #[test]
    fn err_match() {
        let err = Error::Match("invalid match".into());
        assert_eq!(err.to_string(), "invalid match");
    }

    #[test]
    fn err_not_found() {
        let err = Error::NotFound;
        assert_eq!(err.to_string(), "not found");
    }

    #[test]
    fn err_unfinished() {
        let err = Error::UnFinished;
        assert_eq!(err.to_string(), "unfinished");
    }

    #[test]
    fn err_io() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let err = Error::Io(io_err);
        assert_eq!(err.to_string(), "permission denied");
    }

    #[test]
    fn err_io_switch() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: Error = io_err.into();
        assert_eq!(err.to_string(), "file not found");
    }
}
