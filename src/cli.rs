use crate::{config::CliArgs, error::Error};

pub fn args() -> Result<CliArgs, Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err(Error::InvalidArgument {
            r#type: "length".to_string(),
            reason: "args length isn't 2".to_string(),
        });
    }

    let cli_args = CliArgs {
        query: args[1].clone(),
        path: args[2].clone(),
    };

    Ok(cli_args)
}
