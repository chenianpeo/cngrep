use crate::error::Error;
use std::path::PathBuf;

// Arguments parse struct
#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub input_source: Vec<PathBuf>,
    pub mode: Vec<Mode>,
}

// running parameters
#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    CountOnly,
}

// arguments parse result
#[derive(Debug)]
pub enum ParseResult {
    Ok(Config),
    Special(SpecialArgs),
}

// running special args
#[derive(Debug)]
pub enum SpecialArgs {
    Help(&'static str),
    Version(&'static str),
}

impl ParseResult {
    // obtain command line arguments
    // conduct special arguments like empty or help
    pub fn build() -> Result<ParseResult, Error> {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove first no use arg

        if args.is_empty() {
            // return Err(Error::InvalidArg {
            //     r#type: "arguments".into(),
            //     context: "must least 1 arguments".into(),
            // });
            return Err(Error::Argument("must least 1 arguments".into()));
        }

        // judge whether exist special option
        for arg in &args {
            if arg.contains("-h") {
                return Ok(ParseResult::Special(SpecialArgs::Help(help())));
            } else if arg.contains("-v") {
                return Ok(ParseResult::Special(SpecialArgs::Version(version())));
            }
        }

        // obtain user input arguments and parse return Config
        let config = parse_args(&args)?;

        Ok(ParseResult::Ok(config))
    }
}

// parse arguments
fn parse_args(vec: &[String]) -> Result<Config, Error> {
    match vec {
        // match guard
        // running code when satisfy condition statement
        // match one argument, like `cg cngrep`
        // must be pattern when args number is 1
        vec if vec.len() == 1 => Ok(Config {
            pattern: vec[0].clone(),
            input_source: Vec::<PathBuf>::new(),
            mode: Vec::<Mode>::new(),
        }),

        // match two arguments
        // command like, `cg t -c`, `cg t file`,
        // support random two args in [pattern] [path] [option]
        vec if vec.len() == 2 => {
            let mut pattern_no: usize = 0; // judge pattern site
            let mut pattern;
            let mut input_source: Vec<PathBuf> = Vec::new();
            let mut mode: Vec<Mode> = Vec::new();

            // traversal parameter
            for (no, string) in vec.iter().enumerate() {
                // judge input option
                // later can add "--" like "--help"
                if string.starts_with('-') {
                    for i in 1..string.len() {
                        if Some('c') == string.chars().nth(i) && !mode.contains(&Mode::CountOnly) {
                            mode.push(Mode::CountOnly);
                        }
                    }

                    // get other argument
                    // second arg is pattern when first argument is mode
                    if no == pattern_no {
                        pattern_no = 1;
                    }
                }

                // judge input path
                if PathBuf::from(string).is_file() || PathBuf::from(string).is_dir() {
                    input_source.push(PathBuf::from(string));

                    if no == pattern_no {
                        pattern_no = 1;
                    }
                }
            }

            // obtain input pattern
            pattern = vec[pattern_no].clone();

            // when not exist pattern in two arguments
            // default match the entire document
            if !input_source.is_empty() && !mode.is_empty() {
                pattern = "".to_string();
            }

            Ok(Config {
                pattern,
                input_source,
                mode,
            })
        }

        // match three arguments or more
        // support [pattern] [path] [option]
        vec if vec.len() >= 3 => {
            let mut non_pattern: Vec<usize> = Vec::new(); // non-pattern numerical order list
            let mut pattern_no: usize = 0; // pattern numerical order
            let mut input_source: Vec<PathBuf> = Vec::new();
            let mut mode: Vec<Mode> = Vec::new();

            for (no, string) in vec.iter().enumerate() {
                if string.starts_with('-') {
                    for i in 1..string.len() {
                        if Some('c') == string.chars().nth(i) && !mode.contains(&Mode::CountOnly) {
                            mode.push(Mode::CountOnly);
                        }
                    }

                    // push non-pattern no to list
                    non_pattern.push(no);
                }

                // judge whether args is file or dir and push
                if PathBuf::from(string).is_file() || PathBuf::from(string).is_dir() {
                    input_source.push(PathBuf::from(string));
                    non_pattern.push(no);
                }
            }

            // judge pattern site
            for i in 0..vec.len() {
                if !non_pattern.contains(&i) {
                    pattern_no = i;
                    break; // get first non-option parameter as pattern
                }
            }

            Ok(Config {
                pattern: vec[pattern_no].clone(),
                input_source,
                mode,
            })
        }

        // _ => Err(Error::Internal {
        //     context: "Non-Support Arguments".into(),
        // }),
        _ => Err(Error::Argument("Non-Support Arguments".into())),
    }
}

// software version
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// software help information
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
