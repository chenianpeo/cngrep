use std::path::PathBuf;

use crate::error::Error;

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub input_source: Vec<PathBuf>,
    pub mode: Vec<Mode>,
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
    Help(&'static str),
    Version(&'static str),
}

/// obtain software version
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// software help information
fn help() -> &'static str {
    r#"Usage: 
    cngrep [OPTIONS] <PATTERN> [PATH]

Arguments:
    <PATTERN>   Search pattern
    [PATH]      File or directory to search

Options:
    -c, --CountOnly     Count matches only
    "#
}

impl ParseResult {
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

        let config = parse_args(&args)?;
        println!("{:?}", config);

        Ok(ParseResult::Ok(config))
    }
}

/// parse arguments
fn parse_args(vec: &Vec<String>) -> Result<Config, Error> {
    match vec {
        args if vec.len() == 1 => Ok(Config {
            pattern: args[0].clone(),
            input_source: Vec::<PathBuf>::new(),
            mode: Vec::<Mode>::new(),
        }),

        args if vec.len() == 2 => Ok(Config {
            pattern: args[0].clone(),
            input_source: vec![PathBuf::from(args[1].clone())],
            mode: Vec::<Mode>::new(),
        }),

        args if vec.len() >= 3 => {
            let mut path_vec: Vec<PathBuf> = Vec::new();
            for (number, arg) in args.iter().enumerate() {
                if number == 0 || number == (args.len() - 1) {
                    continue;
                }
                path_vec.push(PathBuf::from(arg));
            }
            Ok(Config {
                pattern: args[0].clone(),
                input_source: path_vec,
                mode: vec![Mode::CountOnly],
            })
        }

        _ => Err(Error::Internal {
            context: "Not Finished".into(),
        }),
    }
}

/// parse arguments pattern
fn _parse_pattern(args: &[String]) -> Result<String, Error> {
    let pattern = args[0].clone();

    Ok(pattern)
}

/// parse input source
fn _parse_source(args: &[String]) -> Result<Vec<PathBuf>, Error> {
    let mut path_vec: Vec<PathBuf> = Vec::new();

    if args.len() == 1 {
        return Ok(path_vec);
    }

    let input_source = &args[1];
    let path = PathBuf::from(input_source);
    path_vec.push(path);
    Ok(path_vec)
}

/// parse running mode
fn _parse_mode(args: &[String]) -> Result<Vec<Mode>, Error> {
    let mut mode_vec: Vec<Mode> = Vec::new();

    if args.len() < 3 {
        return Ok(mode_vec);
    }

    let mode = &args[2];
    if mode == "-c" {
        mode_vec.push(Mode::CountOnly);
    }

    Ok(mode_vec)
}
