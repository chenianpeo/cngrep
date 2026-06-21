use crate::config::FileMatch;
use crate::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::StdinLock;

// search content
// current phase that it's very simple
// have different approach for each mode or input source

pub fn match_file_normal(file: BufReader<File>, query: String) -> Result<Vec<FileMatch>, Error> {
    let mut file_match: Vec<FileMatch> = Vec::new();
    for (line_no, line) in file.lines().enumerate() {
        let line = line.map_err(|_| Error::Internal {
            message: "line switch error".to_string(),
        })?;
        let line_no = line_no + 1;
        if line.contains(&query) {
            file_match.push(FileMatch {
                line_no,
                content: line,
            });
        }
    }

    Ok(file_match)
}

pub fn match_file_count(file: BufReader<File>, query: String) -> Result<usize, Error> {
    let mut count: usize = 0;
    for line in file.lines() {
        let line = line.map_err(|_| Error::Internal {
            message: "line switch error".to_string(),
        })?;
        if line.contains(&query) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn match_stdin_normal(buf: StdinLock<'_>, query: String) -> Result<Vec<String>, Error> {
    let mut match_result: Vec<String> = Vec::new();

    for line in buf.lines() {
        let line = line.map_err(|_| Error::Internal {
            message: "line switch error".to_string(),
        })?;

        if line.contains(&query) {
            match_result.push(line);
        }
    }
    Ok(match_result)
}

pub fn match_stdin_count(buf: StdinLock<'_>, query: String) -> Result<usize, Error> {
    let mut count: usize = 0;
    for line in buf.lines() {
        let line = line.map_err(|_| Error::Internal {
            message: "line switch error".to_string(),
        })?;
        if line.contains(&query) {
            count += 1;
        }
    }
    Ok(count)
}
