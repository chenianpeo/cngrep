use crate::error::Error;
use std::path::PathBuf;

/// Arguments parse struct
#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub input_source: Vec<PathBuf>,
    pub mode: Vec<Mode>,
}

/// running parameters
#[derive(Debug, PartialEq)]
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
fn parse_args(vec: &[String]) -> Result<Config, Error> {
    match vec {
        vec if vec.len() == 1 => Ok(Config {
            pattern: vec[0].clone(),
            input_source: Vec::<PathBuf>::new(),
            mode: Vec::<Mode>::new(),
        }),

        // cg t -c, cg t file,
        vec if vec.len() == 2 => {
            let mut pattern_no: usize = 0;
            let mut pattern;
            let mut input_source: Vec<PathBuf> = Vec::new();
            let mut mode: Vec<Mode> = Vec::new();

            for (no, string) in vec.iter().enumerate() {
                //judge whether exist mode
                if Some('-') == string.chars().nth(0) {
                    for i in 1..string.len() {
                        if Some('c') == string.chars().nth(i) && !mode.contains(&Mode::CountOnly) {
                            mode.push(Mode::CountOnly);
                        }
                    }

                    if no == pattern_no {
                        pattern_no = 1;
                    }
                }

                if PathBuf::from(string).is_file() || PathBuf::from(string).is_dir() {
                    input_source.push(PathBuf::from(string));

                    if no == pattern_no {
                        pattern_no = 1;
                    }
                }
            }

            pattern = vec[pattern_no].clone();

            if !input_source.is_empty() && !mode.is_empty() {
                pattern = "".to_string();
            }

            Ok(Config {
                pattern,
                input_source,
                mode,
            })
        }

        // path, multiple file
        vec if vec.len() >= 3 => {
            let mut path_vec: Vec<PathBuf> = Vec::new();
            for (number, arg) in vec.iter().enumerate() {
                if number == 0 || number == (vec.len() - 1) {
                    continue;
                }
                path_vec.push(PathBuf::from(arg));
            }
            Ok(Config {
                pattern: vec[0].clone(),
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
