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

        for arg in &args {
            if arg.contains("-h") {
                return Ok(ParseResult::Special(SpecialArgs::Help(help())));
            } else if arg.contains("-v") {
                return Ok(ParseResult::Special(SpecialArgs::Version(version())));
            }
        }

        if args.len() == 0 {
            return Err(Error::InvalidArg {
                r#type: "arguments".into(),
                context: "must least 1 arguments".into(),
            });
        }

        let pattern = parse_pattern(&args)?;
        let input_source = parse_source(&args)?;
        let mode = parse_mode(&args)?;

        Ok(ParseResult::Ok(Config {
            pattern,
            input_source,
            mode,
        }))
    }
}

/// parse arguments pattern
fn parse_pattern(args: &[String]) -> Result<String, Error> {
    if args.len() < 1 {
        return Err(Error::Internal {
            context: "args number already conduct".to_string(),
        });
    }

    Ok(args[0].clone())
}

/// parse input source
fn parse_source(args: &[String]) -> Result<Option<PathBuf>, Error> {
    if args.len() < 2 {
        return Ok(None);
    } else {
        let path = PathBuf::from(args[1].clone());
        return Ok(Some(path));
    }
}

/// parse running mode
fn parse_mode(args: &[String]) -> Result<Mode, Error> {
    if args.len() < 3 {
        return Ok(Mode::Normal);
    } else if args[2] == "-c" {
        return Ok(Mode::CountOnly);
    } else {
        return Err(Error::Internal {
            context: "don't support this mode".into(),
        });
    }
}
