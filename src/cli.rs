/*
this module is conduct parse and obtain Args
current stage, arguments parse support 1 to 3 and don't support multiple file
arguments parse is very weak, support sequence rather than semantic input
*/

/*
duty:
defined arguments, check argument validity
convert cli arguments to parse structure
unify exit code return
*/

use crate::error::Error;
use std::path::PathBuf;

// parse function
pub fn parse() -> Result<Args, Error> {
    let input_arg: Vec<String> = std::env::args().collect();
    let cli = args(input_arg)?;
    let args = Args::from_cli(cli)?;
    Ok(args)
}

#[derive(Debug)]
pub struct Cli {
    pub query: String,
    pub file: Option<PathBuf>,

    pub count: bool,
}

#[derive(Debug)]
pub struct Args {
    pub query: String,
    pub input_source: InputSource,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum InputSource {
    File(PathBuf),
    Stdin,
    Dir,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    IgnoreCase,
    CountOnly,
}

// conduct arguments input
pub fn args(arg: Vec<String>) -> Result<Cli, crate::error::Error> {
    let mut arg = arg;
    arg.remove(0);

    let help_info = "HELP".to_string();

    match arg.len() {
        1 => {
            if arg[0].clone() == "-h" || arg[0].clone() == "--help" {
                print!("{}", help_info);
            }
            Ok(Cli {
                query: arg[0].clone(),
                file: None,
                count: false,
            })
        }

        2 => {
            if arg[0].clone() == "-c" || arg[0].clone() == "--count-only" {
                Ok(Cli {
                    query: arg[1].clone(),
                    file: None,
                    count: true,
                })
            } else if arg[1].clone() == "-c" || arg[1].clone() == "--count-only" {
                Ok(Cli {
                    query: arg[0].clone(),
                    file: None,
                    count: true,
                })
            } else {
                Ok(Cli {
                    query: arg[0].clone(),
                    file: Some(PathBuf::from(&arg[1])),
                    count: false,
                })
            }
        }

        3 => {
            if arg[0].clone() == "-c" || arg[0].clone() == "--count-only" {
                Ok(Cli {
                    query: arg[1].clone(),
                    file: Some(PathBuf::from(&arg[2])),
                    count: true,
                })
            } else if arg[2].clone() == "-c" || arg[2].clone() == "--count-only" {
                Ok(Cli {
                    query: arg[0].clone(),
                    file: Some(PathBuf::from(&arg[1])),
                    count: true,
                })
            } else {
                Err(Error::InvalidArgument {
                    r#type: "option".to_string(),
                    reason: help_info,
                })
            }
        }

        _ => Err(Error::InvalidArgument {
            r#type: "arguments".to_string(),
            reason: "Command\ncngrep [option] <query> <path>".to_string(),
        }),
    }
}

use std::io::IsTerminal;

// conduct Input source and Input mode.
impl Args {
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        let input_source: InputSource = determine_input(&cli)?;

        let mode: Mode = determine_mode(&cli)?;

        Ok(Self {
            query: cli.query,
            input_source,
            mode,
        })
    }
}

fn determine_input(cli: &Cli) -> Result<InputSource, Error> {
    let input_source = if let Some(file) = &cli.file {
        InputSource::File(file.clone())
    } else if !std::io::stdin().is_terminal() {
        InputSource::Stdin
    } else {
        InputSource::Dir
    };

    Ok(input_source)
}

fn determine_mode(cli: &Cli) -> Result<Mode, Error> {
    let mode = if cli.count {
        Mode::CountOnly
    } else {
        Mode::Normal
    };

    Ok(mode)
}
