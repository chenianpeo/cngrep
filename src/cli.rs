use std::path::PathBuf;

use crate::{
    config::{self, Cli, CliArgs},
    error::Error,
};

pub fn args() -> Result<CliArgs, Error> {
    let args: Vec<String> = std::env::args().collect();

    let mode = config::Mode::Normal;
    match args.len() {
        3 => Ok(CliArgs {
            query: args[1].clone(),
            file: args[2].clone(),
            mode,
        }),
        4 => {
            if args[1].clone() == "-c" {
                let mode = config::Mode::CountOnly;
                Ok(CliArgs {
                    query: args[2].clone(),
                    file: args[3].clone(),
                    mode,
                })
            } else {
                Err(Error::InvalidArgument {
                    r#type: "parse".to_string(),
                    reason: "unknown command".to_string(),
                })
            }
        }
        _ => Err(Error::InvalidArgument {
            r#type: "length".to_string(),
            reason: "args length isn't 2".to_string(),
        }),
    }
}

pub fn _args() -> Result<Cli, crate::error::Error> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        3 => Ok(Cli {
            query: args[1].clone(),
            file: Some(PathBuf::from(&args[2])),
            count: false,
        }),
        _ => Err(Error::InvalidArgument {
            r#type: "arguments".to_string(),
            reason: "failed command".to_string(),
        }),
    }
}
