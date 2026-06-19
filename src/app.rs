use crate::cli::args;
use crate::config::Args;
use crate::error::Error;
use crate::matcher::type_match;

pub fn run() -> Result<(), Error> {
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;

    let args = Args::from_cli(cli)?;

    let match_result = type_match(args)?;
    println!("{:?}", match_result);
    match_result.run()?;

    Ok(())
}
