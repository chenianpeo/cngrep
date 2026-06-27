/*
This module is organize work flow
 */
use crate::cli::parse;
use crate::error::Error;
use crate::matcher::{Match, NeedMatch};
use crate::printer::{NeedPrint, Print};
use crate::reader::ReadSource;

/*
product and feature problem:

# feature incomplete
only have search, not support ignore case, regex, file name, color,
context line, recursion directory, etc

# CLI instability stipulated
arguments parse cannot support system design
option input format don't fit cli habit
*/

/*
architecture layer

# module coupling
hard to test and extent, error handle is confusion

# error module not unified
*/

pub fn run() -> Result<(), Error> {
    // arguments input and parse
    let args = parse()?;

    // read need search content like file or stdin
    // next stage, need add mode match
    // currently, only support file and stdin
    let reader = args.read()?;

    // this struct need optimize
    // provide match basic for search
    let mut need_match = NeedMatch {
        query: args.query.clone(),
        mode: args.mode,
        content: reader,
    };

    // core logic, match content
    // only support normal mode and cannot ignore case
    // this module is method
    let match_result = need_match.search()?;

    // need optimize
    // provide print result and mode control
    let need_print = NeedPrint {
        mode: args.mode,
        result: match_result,
    };

    // output print search result

    // current stage, only support normal and count only mode
    let _ = need_print.print();

    Ok(())
}
