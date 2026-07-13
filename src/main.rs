mod cli;
mod error;
mod reader;

use crate::{
    cli::{ParseResult, SpecialArgs},
    error::Error,
    reader::read,
};

use std::process::ExitCode;

// entry point
fn main() -> ExitCode {
    match run() {
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }

        Ok(_) => ExitCode::from(0),
    }
}

/// # work flow construct
///
/// schedule module running and dispatch
fn run() -> Result<(), Error> {
    let arg = ParseResult::build()?;

    let args = match arg {
        ParseResult::Ok(cfg) => cfg,
        ParseResult::Special(mode) => {
            match mode {
                SpecialArgs::Help(h) => println!("{h}"),
                SpecialArgs::Version(v) => println!("{v}"),
            }
            return Ok(());
        }
    };

    println!("{:?}", args);

    let read_result = read(&args.input_source, &args.mode)?;

    match read_result {
        reader::ReadResult::Stdin(stdin) => println!("{:?}", stdin),
        reader::ReadResult::File(file) => println!("{:?}", file),
        reader::ReadResult::Dir(dir) => println!("{:?}", dir),
    }

    Ok(())
}
