/*
this is the entry point
*/

// Exit Code and Error Handle module
use std::{path::PathBuf, process::ExitCode};

use cngrep::error::_Error;

fn main() -> ExitCode {
    match run() {
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }

        Ok(_) => ExitCode::from(0),
    }
}

#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub input_source: Option<PathBuf>,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum Mode {
    Search,
    File,
}

#[derive(Debug)]
pub enum ParseResult {
    Ok(Config),
    Err(_Error),
    Special(SpecialMode),
}

#[derive(Debug)]
pub enum SpecialMode {
    Help,
    Version,
}

fn run() -> Result<(), _Error> {
    // parse arguments to obtain ParseResult
    let pattern = "pattern".to_string();
    let input_source = None;
    let mode = Mode::Search;

    let args = match ParseResult::Ok(Config {
        pattern,
        input_source,
        mode,
    }) {
        ParseResult::Err(err) => return Err(err),
        ParseResult::Special(_) => return Ok(()),
        ParseResult::Ok(cfg) => cfg,
    };

    // running search
    match &args.mode {
        Mode::Search => (),
        Mode::File => (),
    };

    Ok(())
}
