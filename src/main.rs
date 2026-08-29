use cg::cli::MatchOptions;
use cg::cli::Parse;
use cg::cli::Special;
use cg::error::Error;
use cg::matcher::search;
use cg::printer::output_result;

use std::process::ExitCode;

/// entry point
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::from(0),

        Err(Error::NotFound) => {
            eprintln!("Not Found");
            ExitCode::from(1)
        }

        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(2)
        }
    }
}

/// logical flow
fn run() -> Result<(), Error> {
    let args = Parse::build()?;

    let config = match args {
        Parse::Special(special) => {
            match special {
                Special::Help => new_help(),
                Special::Version => new_version(),
            }

            return Ok(());
        }

        Parse::Ok(config) => config,
    };

    if config.print_config {
        println!("{:#?}", config);
        return Ok(());
    }

    use cg::reader::read;
    let read_result = read(&config.path)?;

    let mut mode = MatchOptions::Normal;
    if config.count && config.ignore_case {
        mode = MatchOptions::IgnoreAndCount
    } else if config.ignore_case {
        mode = MatchOptions::IgnoreCase
    } else if config.count {
        mode = MatchOptions::CountOnly
    }

    let search_result = search(&config.pattern, &read_result, &mode)?;

    output_result(&search_result, config.color)?;

    Ok(())
}

/// help information
fn new_help() {
    let help = include_str!("../docs/help.txt");
    println!("{help}");
}

/// version information
fn new_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("{version}");
}
