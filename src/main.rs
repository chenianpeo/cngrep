use cg::cli::Parse;
use cg::cli::Special;
use cg::error::Error;
use cg::matcher::search;
use cg::printer::output_result;
use cg::reader::read;

use std::process::ExitCode;

/// entry point
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::from(0),

        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}

/// logical flow
fn run() -> Result<(), Error> {
    let config = match Parse::build()? {
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

    let read_result = read(&config.path)?;

    let search_result = search(&config.pattern, &read_result, &config.new_match_mode)?;

    output_result(&search_result, &config.output_mode)?;

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
