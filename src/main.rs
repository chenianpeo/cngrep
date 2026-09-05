use std::process::ExitCode;

use cg::{
    cli::{Parse, Special},
    error::Error,
    matcher::search,
    printer::output,
    reader::read,
};

/// entry point
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::from(0),

        Err(Error::NotFound) => ExitCode::from(1),

        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(2)
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

    let search_result = search(&config.pattern, &read_result, &config.match_mode)?;

    output(&search_result, &config.output_mode)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_output() {
        new_help();
    }

    #[test]
    fn version_output() {
        new_version();
    }
}
