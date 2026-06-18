use std::path::PathBuf;

use crate::{config::Cli, error::Error};
pub fn args() -> Result<Cli, crate::error::Error> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => Ok(Cli {
            query: args[1].clone(),
            file: None,
            count: false,
        }),
        3 => Ok(Cli {
            query: args[1].clone(),
            file: Some(PathBuf::from(&args[2])),
            count: false,
        }),
        4 => {
            if args[1].clone() == "-c" {
                Ok(Cli {
                    query: args[2].clone(),
                    file: Some(PathBuf::from(&args[3])),
                    count: true,
                })
            } else if args[3].clone() == "-c" {
                Ok(Cli {
                    query: args[1].clone(),
                    file: Some(PathBuf::from(&args[2])),
                    count: true,
                })
            } else {
                Err(Error::InvalidArgument {
                    r#type: "option".to_string(),
                    reason: "unknown arguments".to_string(),
                })
            }
        }

        _ => Err(Error::InvalidArgument {
            r#type: "arguments".to_string(),
            reason: "failed command".to_string(),
        }),
    }
}

use std::io::IsTerminal;

use crate::config::{Args, InputSource, Mode};

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
        InputSource::CurrentDir
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
