use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::config::MatchResult;
use crate::error::Error;

pub trait Match {
    fn search(&mut self) -> Result<Vec<MatchResult>, Error>;
}

pub struct FileMatch {
    pub query: String,
    pub file: BufReader<File>,
}

impl Match for FileMatch {
    fn search(&mut self) -> Result<Vec<MatchResult>, Error> {
        let mut search_result: Vec<MatchResult> = Vec::new();

        for (line_no, line) in (&mut self.file).lines().enumerate() {
            let line = line?;
            let line_no = line_no + 1;

            if line.contains(&self.query.to_string()) {
                search_result.push(MatchResult {
                    line_no,
                    content: line,
                })
            }
        }

        Ok(search_result)
    }
}
