use crate::config::FileMatch;
use crate::error::Error;
use crate::printer::DirMatchResult;
use crate::printer::FileMatchResult;
use crate::printer::Print;
use crate::printer::StdinMatchResult;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

// search content
// current phase that it's very simple
// have different approach for each mode or input source

pub trait Match {
    fn search(&mut self) -> Result<Box<dyn Print>, Error>;
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

impl Match for ReadFileResult {
    fn search(&mut self) -> Result<Box<dyn Print>, Error> {
        let mut file_match: Vec<FileMatch> = Vec::new();
        for (line_no, line) in (self.result).by_ref().lines().enumerate() {
            let line = line.map_err(|_| Error::Internal {
                message: "line switch error".to_string(),
            })?;
            let line_no = line_no + 1;
            if line.contains(&self.query) {
                file_match.push(FileMatch {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(Box::new(FileMatchResult {
            content: file_match,
        }))
    }
}

impl<'a> Match for ReadStdinResult<'a> {
    fn search(&mut self) -> Result<Box<dyn Print>, Error> {
        let mut match_result: Vec<String> = Vec::new();

        for line in self.result.by_ref().lines() {
            let line = line.map_err(|_| Error::Internal {
                message: "line switch error".to_string(),
            })?;

            if line.contains(&self.query) {
                match_result.push(line);
            }
        }
        Ok(Box::new(StdinMatchResult {
            content: match_result,
        }))
    }
}

impl Match for ReadDirResult {
    fn search(&mut self) -> Result<Box<dyn Print>, Error> {
        let content = DirMatchResult {};
        Ok(Box::new(content))
    }
}
