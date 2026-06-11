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

#[cfg(test)]
mod error_test {
    use super::*;
    use std::io;

    #[test]
    fn test_io_error() {
        let err = CnError::Io(io::Error::new(io::ErrorKind::Other, "file read failed"));
        assert_eq!(err.to_string(), "io error: file read failed");
    }

    #[test]
    fn test_parse_error() {
        let err = CnError::Parse("invalid arguments length".to_string());
        assert_eq!(err.to_string(), "parse error: invalid arguments length");
    }

    #[test]
    fn test_input_error() {
        let err = CnError::Custom("query content only support word".to_string());
        assert_eq!(
            err.to_string(),
            "invalid input: query content only support word"
        );
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let err: CnError = io_err.into();

        match err {
            CnError::Io(e) => {
                assert_eq!(e.kind(), io::ErrorKind::NotFound);
            }
            _ => panic!("expected CnError::Io"),
        }
    }
}
