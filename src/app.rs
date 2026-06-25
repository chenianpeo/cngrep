/*
This module is organize work flow
 */
use crate::cli::{InputSource, parse};
use crate::error::Error;
use crate::reader::{Read, ReadDir, ReadFile, ReadSource, ReadStdin};

pub fn run() -> Result<(), Error> {
    let args = parse()?;

    let reader = match args.input_source {
        InputSource::File(path) => Read::File(ReadFile {
            query: args.query.clone(),
            path,
        }),
        InputSource::Stdin => Read::Stdin(ReadStdin {
            query: args.query.clone(),
        }),
        InputSource::CurrentDir => Read::Dir(ReadDir {
            query: args.query.clone(),
        }),
    };
    let read_result = reader.read_source()?;

    println!("{:?}", read_result);

    Ok(())
}
