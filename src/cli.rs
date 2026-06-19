use std::path::PathBuf;

use crate::{config::Cli, error::Error};

pub fn args(arg: Vec<String>) -> Result<Cli, crate::error::Error> {
    let mut arg = arg;
    arg.remove(0);
    
    match arg.len() {
        1 => Ok(Cli {
            query: arg[0].clone(),
            file: None,
            count: false,
        }),
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

#[cfg(test)]
mod cli_test {
    use super::*;


    #[test]
    fn test_1_args() {
        let arg: Vec<String> = vec!["q".to_string()];
        let cli = args(arg).unwrap();
        assert_eq!(cli.query, "q".to_string());
        assert_eq!(cli.file, None);
        assert_eq!(cli.count, false);
    }
}
