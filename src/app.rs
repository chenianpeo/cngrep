/*
This module is organize work flow
*/

/* ！
1.  unify data model, only transmit line and config, don't transmit IO
2.  tubular structure, dependency direction is one way
3.  independent module, accept distinct input, can single test, don't mock
    inner type in other layer
    cli -> Config, source -> only create Line, matcher -> pure function,
    printer -> format + write IO
*/

use crate::cli::{_parse, parse};
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

pub fn _run() -> Result<(), Error> {
    // arguments input and parse
    let args = _parse()?;

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

// work process construction
pub fn run() -> Result<(), Error> {
    let _ = parse();

    Ok(())
}
