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
// 0 > success, 1 > not found, 2 > running error
fn main() -> ExitCode {
    let _file = cg::os::file::File::open("/home/cn/Code/cngrep/README.md").unwrap();

    match run() {
        Ok(_) => ExitCode::from(0),

        Err(Error::NotFound(err)) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }

        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(2)
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
    if args
        .special_options
        .contains(&cg::cli::SpecialOptions::PrintConfig)
    {
        println!("{:#?}", args);
    }

    // obtain input source path
    let read_result = read(&args.input_source, &args.read_options)?;

    // match pattern according to mode
    let search_result = search(&args.pattern, &read_result, &args.match_options)?;

    // render and print match result
    render(&args.pattern, &search_result, &args.output_options)?;

    Ok(())
}
