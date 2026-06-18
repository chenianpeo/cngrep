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
