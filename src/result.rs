/*
This module is build result for matcher
that have once match, total match and build function, etc
use collect from search result to data structure
 */

// use std::{fs::File, path::PathBuf};
// use crate::{error::Error, printer::Print};

// #[derive(Debug)]
// pub struct FileMatch {
//     pub line_no: usize,
//     pub content: String,
// }

// #[derive(Debug)]
// pub struct StdinMatch {
//     pub content: String,
// }

// pub struct DirMatch {
//     pub file: PathBuf,
//     pub result: Vec<FileMatch>,
// }

// pub trait Collect {
//     fn collect(self) -> Result<Box<dyn Print>, Error>;
// }

// impl Collect for Vec<FileMatch> {
//     fn collect(self) -> Result<Box<dyn Print>, Error> {
//         let result = Box::new(self);

//         Ok(result)
//     }
// }
