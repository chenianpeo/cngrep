use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub enum Read {
    File(ReadFile),
    Stdin(ReadStdin),
    Dir(ReadDir),
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

#[derive(Debug)]
pub enum ReadResult<'a> {
    File(&'a ReadFile),
    Stdin(&'a ReadStdin),
    Dir(&'a ReadDir),
}

#[derive(Debug)]
pub struct ReadFileResult {
    pub query: String,
    pub result: BufReader<File>,
}

#[derive(Debug)]
pub struct ReadStdinResult<'a> {
    pub query: String,
    pub result: std::io::StdinLock<'a>,
}

#[derive(Debug)]
pub struct ReadDirResult {
    pub query: String,
}

pub trait ReadSource {
    fn read_source(&self) -> Result<ReadResult<'_>, Error>;
}

impl ReadSource for Read {
    fn read_source(&self) -> Result<ReadResult<'_>, Error> {
        let result = match self {
            Self::File(read_file) => ReadResult::File(read_file),
            Self::Stdin(read_stdin) => ReadResult::Stdin(read_stdin),
            Self::Dir(read_dir) => ReadResult::Dir(read_dir),
        };

        Ok(result)
    }
}

// impl ReadSource for ReadFile {
//     fn read_source(&self) -> Result<Box<dyn Match>, Error> {
//         let query = self.query.clone();

//         let file = File::open(&self.path).map_err(|_| Error::Io {
//             source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found target file"),
//             context: Some("File read error".to_string()),
//         })?;
//         let result = BufReader::new(file);
//         let read = ReadFileResult { query, result };
//         Ok(Box::new(read))
//     }
// }

// impl ReadSource for ReadStdin {
//     fn read_source(&self) -> Result<Box<dyn Match>, Error> {
//         let query = self.query.clone();

//         let buf = std::io::stdin();
//         let result = buf.lock();
//         let read = ReadStdinResult { query, result };
//         Ok(Box::new(read))
//     }
// }

// impl ReadSource for ReadDir {
//     fn read_source(&self) -> Result<Box<dyn Match>, Error> {
//         let query = self.query.clone();
//         let cwd = env::current_dir().unwrap();
//         println!("{}", cwd.display());

//         let read = ReadDirResult { query };
//         Ok(Box::new(read))
//     }
// }
