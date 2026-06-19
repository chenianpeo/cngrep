use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::config::InputSource;
use crate::config::Mode;
use crate::error::Error;

// pub trait Match {
//     fn search_normal(&mut self) -> Result<Vec<MatchResult>, Error>;
//     fn search_count_only(&mut self) -> Result<usize, Error>;
// }

pub struct FileMatch {
    pub query: String,
    pub file: BufReader<File>,
}

// impl Match for FileMatch {
//     fn search_normal(&mut self) -> Result<Vec<MatchResult>, Error> {
//         let mut search_result: Vec<MatchResult> = Vec::new();

//         for (line_no, line) in (&mut self.file).lines().enumerate() {
//             let line = line?;
//             let line_no = line_no + 1;

//             if line.contains(&self.query) {
//                 search_result.push(MatchResult {
//                     line_no,
//                     content: line,
//                 })
//             }
//         }

//         Ok(search_result)
//     }

//     fn search_count_only(&mut self) -> Result<usize, Error> {
//         let mut count: usize = 0;

//         for line in (&mut self.file).lines() {
//             let line = line.unwrap();
//             if line.contains(&self.query) {
//                 count += 1;
//             }
//         }

//         Ok(count)
//     }
// }

#[derive(Debug)]
pub struct NormalFile {
    pub query: String,
    pub file: PathBuf,
}

pub struct CountFile {
    pub query: String,
    pub file: PathBuf
}

// #[derive(Debug)]
// pub enum Type {
//     File(NormalFile)
// }
use crate::config::Args;
use crate::reader::Read;
// use crate::matcher::Type::NormalFile;
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

// pub trait Read {
//     fn read();
// }