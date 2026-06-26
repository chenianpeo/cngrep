/*
This module is organize work flow
 */
use crate::cli::parse;
use crate::error::Error;
use crate::matcher::{Match, NeedMatch};
use crate::printer::Print;
use crate::reader::ReadSource;

pub fn run() -> Result<(), Error> {
    let args = parse()?;

    let reader = args.read()?;

    let mut need_match = NeedMatch {
        query: args.query.clone(),
        content: reader,
    };

    let match_result = need_match.search()?;

    let _ = match_result.print();

    Ok(())
}
