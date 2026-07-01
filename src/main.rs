mod cli;
mod error;

use crate::{
    cli::{ParseResult, SpecialArgs},
    error::Error,
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

    println!("{} {:?} {:?}", args.pattern, args.input_source, args.mode);

    Ok(())
}
