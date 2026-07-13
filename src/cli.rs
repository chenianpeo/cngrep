use std::path::PathBuf;

use crate::error::Error;

/// cli arguments config
#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub input_source: Option<PathBuf>,
    pub mode: Mode,
}

/// running parameters
#[derive(Debug)]
pub enum Mode {
    Normal,
    CountOnly,
}

/// arguments parse result
#[derive(Debug)]
pub enum ParseResult {
    Ok(Config),
    Special(SpecialArgs),
}

/// running special args
#[derive(Debug)]
pub enum SpecialArgs {
    Help(String),
    Version(String),
}

use std::fmt::Write;
/// obtain software version
fn version() -> String {
    let mut version = String::new();
    let _ = write!(version, "ripgrep: {}", env!("CARGO_PKG_VERSION"));
    version
}

/// software help information
fn help() -> String {
    "Help: ".to_string()
}

impl ParseResult {
    /// parse arguments
    pub fn build() -> Result<ParseResult, Error> {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);

        if args.is_empty() {
            return Err(Error::InvalidArg {
                r#type: "arguments".into(),
                context: "must least 1 arguments".into(),
            });
        }

        for arg in &args {
            if arg.contains("-h") {
                return Ok(ParseResult::Special(SpecialArgs::Help(help())));
            } else if arg.contains("-v") {
                return Ok(ParseResult::Special(SpecialArgs::Version(version())));
            }
        }

        let pattern = parse_pattern(&args)?;
        let input_source = parse_source(&args)?;
        let mode = parse_mode(&args)?;

        let config = Config {
            pattern,
            input_source,
            mode,
        };

        Ok(ParseResult::Ok(config))
    }
}

/// parse arguments pattern
fn parse_pattern(args: &[String]) -> Result<String, Error> {
    Ok(args[0].clone())
}

/// parse input source
fn parse_source(args: &[String]) -> Result<Option<PathBuf>, Error> {
    let Some(path) = args.get(1) else {
        return Ok(None);
    };

    let path = PathBuf::from(path);
    Ok(Some(path))
}

/// parse running mode
fn parse_mode(args: &[String]) -> Result<Mode, Error> {
    if args.len() < 3 {
        Ok(Mode::Normal)
    } else if args[2] == "-c" {
        Ok(Mode::CountOnly)
    } else {
        Err(Error::Internal {
            context: "don't support this mode".into(),
        })
    }
}
