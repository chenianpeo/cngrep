use cngrep::matcher::search;

use cngrep::{
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

    let mut read_result = read(&args.input_source, &args.mode)?;

    let _ = search(&args.pattern, &mut read_result, &args.mode);

    Ok(())
}
