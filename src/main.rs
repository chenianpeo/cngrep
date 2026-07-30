use cg::cli::ParseResult;
use cg::cli::SpecialArgs;
use cg::error::Error;
use cg::matcher::search;
use cg::printer::render;
use cg::reader::read;

use std::process::ExitCode;

// entry point
// right return match result else return exit code
// todo: exit code should design to be, 0123
// 0 > success, 1 > not found, 2 > args error
// 3 > IO error
fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::from(0),

        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}

/// # work flow construct
///
/// schedule module running and dispatch
fn run() -> Result<(), Error> {
    // parse cli arguments
    let arg = ParseResult::build()?;

    // match parse result and obtain config
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

    // obtain input source path
    let read_result = read(&args.input_source, &args.mode)?;

    // match pattern according to mode
    let search_result = search(&args.pattern, &read_result, &args.mode)?;

    // render and print match result
    render(&args.pattern, &search_result, &args.mode)?;

    Ok(())
}
