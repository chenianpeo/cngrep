use crate::error::Error;
use crate::matcher::{Match, ReadDirResult, ReadFileResult, ReadStdinResult};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// read file and stdin content
// need different approach for each type
pub trait Read {
    fn read(&self) -> Result<Box<dyn Match>, Error>;
}

#[derive(Debug)]
pub struct ReadFile {
    pub query: String,
    pub path: PathBuf,
}

#[derive(Debug)]
pub struct ReadDir {
    pub query: String,
}

#[derive(Debug)]
pub struct ReadStdin {
    pub query: String,
}

impl Read for ReadFile {
    fn read(&self) -> Result<Box<dyn Match>, Error> {
        let query = self.query.clone();

        let file = File::open(&self.path).map_err(|_| Error::Io {
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found target file"),
            context: Some("File read error".to_string()),
        })?;
        let result = BufReader::new(file);
        let read = ReadFileResult { query, result };
        Ok(Box::new(read))
    }
}

impl Read for ReadStdin {
    fn read(&self) -> Result<Box<dyn Match>, Error> {
        let query = self.query.clone();

        let buf = std::io::stdin();
        let result = buf.lock();
        let read = ReadStdinResult { query, result };
        Ok(Box::new(read))
    }
}

impl Read for ReadDir {
    fn read(&self) -> Result<Box<dyn Match>, Error> {
        let query = self.query.clone();
        let cwd = env::current_dir().unwrap();
        println!("{}", cwd.display());

        let read = ReadDirResult { query };
        Ok(Box::new(read))
    }
}
