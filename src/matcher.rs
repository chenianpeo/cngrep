use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::config::MatchResult;
use crate::error::Error;

pub trait Match {
    fn search_normal(&mut self) -> Result<Vec<MatchResult>, Error>;
    fn search_count_only(&mut self) -> Result<usize, Error>;
}

pub struct FileMatch {
    pub query: String,
    pub file: BufReader<File>,
}

impl Match for FileMatch {
    fn search_normal(&mut self) -> Result<Vec<MatchResult>, Error> {
        let mut search_result: Vec<MatchResult> = Vec::new();

        for (line_no, line) in (&mut self.file).lines().enumerate() {
            let line = line?;
            let line_no = line_no + 1;

            if line.contains(&self.query) {
                search_result.push(MatchResult {
                    line_no,
                    content: line,
                })
            }
        }

        Ok(search_result)
    }

    fn search_count_only(&mut self) -> Result<usize, Error> {
        let mut count: usize = 0;

        for line in (&mut self.file).lines() {
            let line = line.unwrap();
            if line.contains(&self.query) {
                count = count +1;
            }
        }

        Ok(count)
    }
}

pub trait SearchMode {
    fn search_mode() {}
}