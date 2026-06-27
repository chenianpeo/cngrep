/*
this module is read search content
include single file, stdin, directory and multiple file
but current stage only support single file and stdin
*/

use crate::cli::{Args, InputSource};
use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ReadFileResult {
    pub path: PathBuf,
    pub result: BufReader<File>,
}

#[derive(Debug)]
pub struct ReadStdinResult<'a> {
    pub result: std::io::StdinLock<'a>,
}

#[derive(Debug)]
pub struct ReadDirResult {
    pub result: Vec<ReadFileResult>,
}

#[derive(Debug)]
pub enum ReadResult<'a> {
    File(ReadFileResult),
    Stdin(ReadStdinResult<'a>),
    Dir(ReadDirResult),
}

pub trait ReadSource {
    fn read(&self) -> Result<ReadResult<'_>, Error>;
}

impl ReadSource for Args {
    fn read(&self) -> Result<ReadResult<'_>, Error> {
        let input_source = &self.input_source;

        let reader = match input_source {
            InputSource::File(path) => {
                let file = File::open(path).map_err(|_| Error::Io {
                    source: std::io::Error::new(std::io::ErrorKind::NotFound, "file not found"),
                    context: Some("read file report error".into()),
                })?;
                let result = BufReader::new(file);

                Ok(ReadResult::File(ReadFileResult {
                    path: path.to_path_buf(),
                    result,
                }))
            }

            InputSource::Stdin => {
                let buf = std::io::stdin();
                let result = buf.lock();

                Ok(ReadResult::Stdin(ReadStdinResult { result }))
            }

            InputSource::Dir => Err(Error::Internal {
                message: "Read Dir Unfinished".to_string(),
            }),
        }?;

        Ok(reader)
    }
}
