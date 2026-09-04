use std::path::PathBuf;

use crate::{error::Error, matcher::MatchMode, printer::OutputMode};

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

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub path: Vec<PathBuf>,
    pub print_config: bool,
    pub output_mode: OutputMode,
    pub new_match_mode: MatchMode,
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

    let output_mode = OutputMode {
        color: cli.color,
        line_num: cli.line_num,
    };

    let match_mode = MatchMode {
        count: cli.count,
        ignore_case: cli.ignore_case,
    };

    let config = Config {
        pattern: cli.pattern,
        path: cli.path,
        print_config: cli.print_config,
        output_mode,
        new_match_mode: match_mode,
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
    pub line_num: bool,
}
