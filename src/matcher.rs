use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::config::InputSource;
use crate::config::Mode;
use crate::error::Error;


pub struct FileMatch {
    pub query: String,
    pub file: BufReader<File>,
}

#[derive(Debug)]
pub struct NormalFile {
    pub query: String,
    pub file: PathBuf,
}

pub struct CountFile {
    pub query: String,
    pub file: PathBuf
}

use crate::config::Args;
use crate::reader::Read;
pub fn type_match(args: Args) -> Result<Box<dyn Type>, Error> {
    match (args.input_source, args.mode) {
        (InputSource::File(path), Mode::Normal) => Ok(Box::new(NormalFile {query: args.query, file: path})),
        (InputSource::File(path), Mode::CountOnly) => Ok(Box::new(NormalFile {query: args.query, file: path})),
        _ => Err(Error::Internal {
            message: "type match report error".to_string(),
        }),
    }
}

pub trait Type: Debug  {
    fn run(&self) -> Result<(), Error>;
}

impl Type for NormalFile {
    fn run(&self) -> Result<(), Error>{
        self.read();

        Ok(())
    }
}
