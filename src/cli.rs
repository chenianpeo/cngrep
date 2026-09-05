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
    pub match_mode: MatchMode,
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

        let config = parse(Cli::parse())?;
        Ok(Parse::Ok(config))
    }
}

fn parse(cli: Cli) -> Result<Config, Error> {
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
        match_mode,
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

    #[arg(short = 'n', long = "line-num")]
    pub line_num: bool,
}

/// test config parse
/// later, need complete cli parse
#[cfg(test)]
mod cli_tests {
    use std::vec;

use super::*;

    #[test]
    fn test_parse_default() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![],
            print_config: false,
            count: false,
            ignore_case: false,
            color: false,
            line_num: false
        };

        let config = parse(cli).unwrap();

        assert_eq!(config.pattern, "hello");
        assert!(config.path.is_empty());

        assert!(!config.print_config);

        assert!(!config.output_mode.color);
        assert!(!config.output_mode.line_num);

        assert!(!config.match_mode.count);
        assert!(!config.match_mode.ignore_case);
    }

    #[test]
    fn test_parse_pattern() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![],
            print_config: false,
            count: false,
            ignore_case: false,
            color: false,
            line_num: false
        };

        let config = parse(cli).unwrap();
        assert_eq!(config.pattern, "hello");
    }

    #[test]
    fn test_parse_path() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![
                PathBuf::from("src"),
                PathBuf::from("tests"),
                PathBuf::from("README.md"),
            ],
            print_config: false,
            count: false,
            ignore_case: false,
            color: false,
            line_num: false
        };

        let config = parse(cli).unwrap();
        assert_eq!(
            config.path,
            vec![
                PathBuf::from("src"),
                PathBuf::from("tests"),
                PathBuf::from("README.md"),
            ]
        )
    }

    #[test]
    fn test_parse_match_mode() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![],
            print_config: false,
            count: true,
            ignore_case: true,
            color: false,
            line_num: false
        };

        let config = parse(cli).unwrap();
        assert!(config.match_mode.count);
        assert!(config.match_mode.ignore_case);
    }

    #[test]
    fn test_parse_output_mode() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![],
            print_config: false,
            count: false,
            ignore_case: false,
            color: true,
            line_num: true,
        };

        let config = parse(cli).unwrap();
        assert!(config.output_mode.color);
        assert!(config.output_mode.line_num);
    }

    #[test]
    fn test_parse_all_options() {
        let cli = Cli {
            pattern: "hello".into(),
            path: vec![
                PathBuf::from("src"),
                PathBuf::from("tests"),
            ],
            print_config: true,
            count: true,
            ignore_case: false,
            color: true,
            line_num: false,
        };

        let config = parse(cli).unwrap();
        
        assert_eq!(config.pattern, "hello");

        assert_eq!(
            config.path,
            vec![
                PathBuf::from("src"),
                PathBuf::from("tests"),
            ]
        );

        assert!(config.print_config);

        assert!(config.match_mode.count);
        assert!(!config.match_mode.ignore_case);

        assert!(config.output_mode.color);
        assert!(!config.output_mode.line_num);
    }
}