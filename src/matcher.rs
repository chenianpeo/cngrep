/*
this module is match search result
obtain result by match query and content line
current stage, don't support anything mode
need optimize from the beginning
*/

/*
matching engine
running pattern match and return result
pre-compile pattern, avoid repeat build
abstract engine such as normal or regex
*/

/*
Core:
only responsible calculate, don't include input and output
*/

use std::io::{BufRead, Read};

use crate::cli::Mode;
use crate::error::Error;
use crate::result::{FileMatch, StdinMatch};
use crate::{reader::ReadResult, result::MatchResult};

#[derive(Debug)]
pub struct NeedMatch<'a> {
    pub query: String,
    pub mode: Mode,
    pub content: ReadResult<'a>,
}

pub trait Match {
    fn search(&mut self) -> Result<MatchResult, Error>;
}

impl<'a> Match for NeedMatch<'a> {
    fn search(&mut self) -> Result<MatchResult, Error> {
        let query = &self.query;

        let result = match &mut self.content {
            ReadResult::File(file) => {
                let mut match_result: Vec<FileMatch> = Vec::new();

                for (line_no, line) in file.result.by_ref().lines().enumerate() {
                    let line = line?;
                    let line_no = line_no + 1;

                    if line.contains(query) {
                        match_result.push(FileMatch {
                            path: file.path.to_path_buf(),
                            line_no,
                            content: line,
                        });
                    }
                }

                Ok(MatchResult::File(match_result))
            }

            ReadResult::Stdin(file) => {
                let mut match_result: Vec<StdinMatch> = Vec::new();

                for line in file.result.by_ref().lines() {
                    let line = line?;

                    if line.contains(query) {
                        match_result.push(StdinMatch { content: line });
                    }
                }

                Ok(MatchResult::Stdin(match_result))
            }

            ReadResult::Dir(_) => Err(Error::Internal {
                message: "Search Dir Unfinished".into(),
            }),
        }?;

        Ok(result)
    }
}
