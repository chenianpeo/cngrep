use crate::error::Error;
use std::{fs::File, path::PathBuf};

// Arguments parse struct
#[derive(Debug, PartialEq)]
pub struct Config {
    pub pattern: String,
    pub input_source: Vec<PathBuf>,
    pub read_options: Vec<ReadOptions>,
    pub match_options: Vec<MatchOptions>,
    pub output_options: Vec<OutputOptions>,
    pub special_options: Vec<SpecialOptions>,
}

#[derive(Default, Debug, PartialEq)]
pub enum ReadOptions {
    #[default]
    Normal,
}

#[derive(Default, Debug, PartialEq)]
pub enum MatchOptions {
    #[default]
    Normal,

    CountOnly,
    IgnoreCase,
}

#[derive(Default, Debug, PartialEq)]
pub enum OutputOptions {
    #[default]
    Terminal,

    File(PathBuf),
}

#[derive(Debug, PartialEq)]
pub enum SpecialOptions {
    Help,
    PrintConfig,
    Version,
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

// #[derive(Debug)]
// pub enum Parse {
//     Ok(Config),
//     Special(SpecialOptions),
// }

impl ParseResult {
    // obtain command line arguments
    // conduct special arguments like empty or help
    pub fn build() -> Result<ParseResult, Error> {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0); // remove first no use arg

        if args.is_empty() {
            return Err(Error::Argument("must least 1 arguments".into()));
        }

        // judge whether exist special option
        for arg in &args {
            if arg == "-h" || arg == "--help" {
                return Ok(ParseResult::Special(SpecialArgs::Help(help())));
            }

            if arg == "-v" || arg == "--version" {
                return Ok(ParseResult::Special(SpecialArgs::Version(version())));
            }
        }

        // obtain user input arguments and parse return Config
        let config = parse_args(&args)?;

        Ok(ParseResult::Ok(config))
    }
}

// TODO refactor parse flow
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
            read_options: vec![],
            match_options: vec![],
            output_options: vec![],
            special_options: vec![],
        }),

        // match two arguments
        // command like, `cg t -c`, `cg t file`,
        // support random two args in [pattern] [path] [option]
        vec if vec.len() == 2 => {
            let mut pattern_no: usize = 0; // judge pattern site
            let mut pattern;

            let mut input_source: Vec<PathBuf> = Vec::new();

            let read_options: Vec<ReadOptions> = Vec::new();
            let mut match_options: Vec<MatchOptions> = Vec::new();
            let output_options: Vec<OutputOptions> = Vec::new();
            let mut special_options: Vec<SpecialOptions> = Vec::new();

            // traversal parameter
            for (no, string) in vec.iter().enumerate() {
                // judge input option
                // later can add "--" like "--help"
                if string.starts_with('-') {
                    for i in 1..string.len() {
                        if Some('c') == string.chars().nth(i)
                            && !match_options.contains(&MatchOptions::CountOnly)
                        {
                            match_options.push(MatchOptions::CountOnly);
                        }

                        if Some('p') == string.chars().nth(i)
                            && !special_options.contains(&SpecialOptions::PrintConfig)
                        {
                            special_options.push(SpecialOptions::PrintConfig);
                        }

                        if Some('i') == string.chars().nth(i)
                            && !match_options.contains(&MatchOptions::IgnoreCase)
                        {
                            match_options.push(MatchOptions::IgnoreCase);
                        }

                        if Some('o') == string.chars().nth(i)
                            && !special_options.contains(&SpecialOptions::PrintConfig)
                        {
                            special_options.push(SpecialOptions::PrintConfig);
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
            if !input_source.is_empty()
                && (!read_options.is_empty()
                    || !match_options.is_empty()
                    || !output_options.is_empty()
                    || !special_options.is_empty())
            {
                pattern = "".to_string();
            }

            Ok(Config {
                pattern,
                input_source,
                read_options,
                match_options,
                output_options,
                special_options,
            })
        }

        // match three arguments or more
        // support [pattern] [path] [option]
        vec if vec.len() >= 3 => {
            let mut non_pattern: Vec<usize> = Vec::new(); // non-pattern numerical order list
            let mut pattern_no: usize = 0; // pattern numerical order

            let mut input_source: Vec<PathBuf> = Vec::new();

            let mut o_flag = 0;

            let read_options: Vec<ReadOptions> = Vec::new();
            let mut match_options: Vec<MatchOptions> = Vec::new();
            let mut output_options: Vec<OutputOptions> = Vec::new();
            let mut special_options: Vec<SpecialOptions> = Vec::new();

            for (no, string) in vec.iter().enumerate() {
                if string.starts_with('-') {
                    for i in 1..string.len() {
                        if Some('c') == string.chars().nth(i)
                            && !match_options.contains(&MatchOptions::CountOnly)
                        {
                            match_options.push(MatchOptions::CountOnly);
                        }

                        let output_file = PathBuf::new();

                        if Some('o') == string.chars().nth(i)
                            && !output_options.contains(&OutputOptions::File(output_file))
                        {
                            let output_file = PathBuf::from(vec[no + 1].clone());

                            if !output_file.exists() {
                                File::create(&output_file)?;
                                o_flag = no + 1;
                                output_options.push(OutputOptions::File(output_file.clone()));
                            }

                            if output_file.is_file() {
                                o_flag = no + 1;
                                output_options.push(OutputOptions::File(output_file))
                            }
                        }

                        if Some('p') == string.chars().nth(i)
                            && !special_options.contains(&SpecialOptions::PrintConfig)
                        {
                            special_options.push(SpecialOptions::PrintConfig);
                        }

                        if Some('i') == string.chars().nth(i)
                            && !match_options.contains(&MatchOptions::IgnoreCase)
                        {
                            match_options.push(MatchOptions::IgnoreCase);
                        }
                    }

                    // push non-pattern no to list
                    non_pattern.push(no);
                }

                if no == o_flag {
                    non_pattern.push(no);
                    continue;
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
                read_options,
                match_options,
                output_options,
                special_options,
            })
        }

        _ => Err(Error::Argument("Non-Support Arguments".into())),
    }
}

// software version
fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// software help information
fn help() -> &'static str {
    include_str!("../docs/help.txt")
}

// Unit Test
// only test one module or one function
// unit test does not rely on the entire program

#[derive(Default, Debug)]
pub struct NewConfig {
    pub pattern: String,
    pub path: Vec<PathBuf>,
    pub print_config: bool,
    pub count: bool,
    pub ignore_case: bool,
    pub color: bool,
}

#[derive(Debug)]
pub enum Special {
    Help,
    Version,
}

#[derive(Debug)]
pub enum Parse {
    Ok(NewConfig),
    Sp(Special),
}

impl Parse {
    /// build `Parse` include Config or Special
    pub fn build() -> Result<Parse, Error> {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);

        if args.is_empty() {
            return Err(Error::Argument("must least 1 argument".into()));
        }

        for arg in &args {
            if arg == "-h" || arg == "--help" {
                return Ok(Parse::Sp(Special::Help));
            }

            if arg == "-v" || arg == "--version" {
                return Ok(Parse::Sp(Special::Version));
            }
        }

        let config = parse(&args)?;
        Ok(Parse::Ok(config))
    }
}

fn parse(_args: &[String]) -> Result<NewConfig, Error> {
    let cli = Cli::parse();

    let config = NewConfig {
        pattern: cli.pattern,
        path: cli.path,
        print_config: cli.print_config,
        count: cli.count,
        ignore_case: cli.ignore_case,
        color: cli.color,
    };
    Ok(config)
}

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cg")]
pub struct Cli {
    pub pattern: String,

    pub path: Vec<PathBuf>,

    #[arg(long)]
    pub print_config: bool,

    #[arg(short, long)]
    pub count: bool,

    #[arg(short, long)]
    pub ignore_case: bool,

    #[arg(long)]
    pub color: bool,
}
