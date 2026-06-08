use crate::config::CliArgs;

pub fn args() -> Result<CliArgs, String> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        return Err("arguments length not enough".to_string());
    }

    let cli_args = CliArgs {
        query: args[1].clone(),
        path: args[2].clone(),
    };

    Ok(cli_args)
}
