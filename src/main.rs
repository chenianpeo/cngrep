use cg::cli::MatchOptions;
// use cg::cli::MatchOptions;
// use cg::cli::ParseResult;
// use cg::cli::SpecialArgs;
// use cg::matcher::new_search;
// use cg::printer::output_result;
// use cg::reader::read;
use cg::cli::Parse;
use cg::cli::Special;
use cg::error::Error;
use cg::matcher::new_search;
use cg::printer::output_result;

use std::process::ExitCode;

// entry point
// right return match result else return exit code
// todo: exit code should design to be, 0123
// 0 > success, 1 > not found, 2 > running error
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

// # work flow construct
//
// schedule module running and dispatch
// fn _run() -> Result<(), Error> {
//     // parse cli arguments
//     let arg = ParseResult::build()?;

//     // match parse result and obtain config
//     let args = match arg {
//         ParseResult::Ok(cfg) => cfg,
//         ParseResult::Special(mode) => {
//             match mode {
//                 SpecialArgs::Help(h) => println!("{h}"),
//                 SpecialArgs::Version(v) => println!("{v}"),
//             }
//             return Ok(());
//         }
//     };
//     if args
//         .special_options
//         .contains(&cg::cli::SpecialOptions::PrintConfig)
//     {
//         println!("{:#?}", args);
//     }

//     // obtain input source path
//     let read_result = read(&args.input_source, &args.read_options)?;

//     // match pattern according to mode
//     let mut mode = MatchOptions::Normal;

//     if args.match_options.contains(&MatchOptions::CountOnly) {
//         mode = MatchOptions::CountOnly
//     } else if args.match_options.contains(&MatchOptions::IgnoreCase) {
//         mode = MatchOptions::IgnoreCase
//     }

//     let search_result = new_search(&args.pattern, &read_result, &mode)?;

//     // render and print match result
//     output_result(&search_result)?;

//     Ok(())
// }

/// logical flow
fn run() -> Result<(), Error> {
    // parse command line arguments
    // return `Parse` result
    let args = Parse::build()?;

    let config = match args {
        Parse::Sp(special) => {
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

    let search_result = new_search(&config.pattern, &read_result, &mode)?;

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
