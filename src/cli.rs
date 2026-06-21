use std::path::PathBuf;

use crate::{config::Cli, error::Error};

// input arguments
// switch to type Struct `Cli` by method
pub fn args(arg: Vec<String>) -> Result<Cli, crate::error::Error> {
    let mut arg = arg;
    arg.remove(0);

    let help_info = "Not Command\n
Example Command:
cngrep [Option] Pattern [Path...]\n
[Option]\n
    default                 print search result content
    --count-only, -c,       print search line number

Pattern\n
[Path...]\n
"
    .to_string();

    match arg.len() {
        // process simply handle
        // existed pattern: arguments number is 1 to 3
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
            reason: help_info,
        }),
    }
}

use std::io::IsTerminal;

use crate::config::{Args, InputSource, Mode};

impl Args {
    // associate function
    // is method if have `self`
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        // decide input source such as Stdin or File
        let input_source: InputSource = determine_input(&cli)?;

        // decide input mode such as Normal or Count Only
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
        let arg: Vec<String> = vec!["da".to_string(), "q".to_string()];
        let cli = args(arg).unwrap();
        assert_eq!(cli.query, "q".to_string());
        assert_eq!(cli.file, None);
        assert_eq!(cli.count, false);
    }
}
