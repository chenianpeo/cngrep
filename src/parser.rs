use std::io::IsTerminal;

use crate::{
    config::{Args, Cli, InputSource, Mode},
    error::Error,
};

impl Args {
    pub fn from_cli(cli: Cli) -> Result<Self, Error> {
        let input_source: InputSource = determine_input(&cli)?;

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
