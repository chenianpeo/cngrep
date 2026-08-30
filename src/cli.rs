use crate::error::Error;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum MatchOptions {
    Normal,
    CountOnly,
    IgnoreCase,
    IgnoreAndCount,
}

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub path: Vec<PathBuf>,
    pub print_config: bool,
    pub color: bool,
    pub line_no: bool,
    pub match_mode: MatchOptions,
}

#[derive(Debug)]
pub enum Parse {
    Ok(Config),
    Special(Special),
}

#[derive(Debug)]
pub enum Special {
    Help,
    Version,
}

impl Parse {
    pub fn build() -> Result<Parse, Error> {
        let mut args: Vec<String> = std::env::args().collect();
        args.remove(0);

        if args.is_empty() {
            return Err(Error::Argument("must least 1 argument".into()));
        }

        for arg in &args {
            if arg == "-h" || arg == "--help" {
                return Ok(Parse::Special(Special::Help));
            }

            if arg == "-v" || arg == "--version" {
                return Ok(Parse::Special(Special::Version));
            }
        }

        let config = parse(&args)?;
        Ok(Parse::Ok(config))
    }
}

fn parse(_args: &[String]) -> Result<Config, Error> {
    let cli = Cli::parse();

    let mut mode = MatchOptions::Normal;
    if cli.count && cli.ignore_case {
        mode = MatchOptions::IgnoreAndCount
    } else if cli.ignore_case {
        mode = MatchOptions::IgnoreCase
    } else if cli.count {
        mode = MatchOptions::CountOnly
    }

    let config = Config {
        pattern: cli.pattern,
        path: cli.path,
        print_config: cli.print_config,
        color: cli.color,
        line_no: cli.line_no,
        match_mode: mode,
    };
    Ok(config)
}

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cg")]
pub struct Cli {
    pub pattern: String,

    pub path: Vec<PathBuf>,

    #[arg(long = "print-config")]
    pub print_config: bool,

    #[arg(short, long)]
    pub count: bool,

    #[arg(short, long = "ignore-case")]
    pub ignore_case: bool,

    #[arg(long)]
    pub color: bool,

    #[arg(long = "line-number")]
    pub line_no: bool,
}
