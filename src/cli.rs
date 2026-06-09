use crate::{config::CliArgs, error::CnError};

pub fn args() -> Result<CliArgs, CnError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err(CnError::Parse("arguments length not enough".to_string()));
    }

    let cli_args = CliArgs {
        query: args[1].clone(),
        path: args[2].clone(),
    };

    Ok(cli_args)
}
