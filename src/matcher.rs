use std::{
    fs::File,
    io::{BufRead, BufReader, Stdin},
    path::PathBuf,
};

use crate::{
    cli::Mode,
    error::Error,
    reader::ReadResult,
    result::{MatchDir, MatchFile, MatchResult, MatchStdin},
};

pub fn search(
    pattern: &str,
    read_result: &ReadResult,
    _mode: &[Mode],
) -> Result<MatchResult, Error> {
    let result: MatchResult = match read_result {
        ReadResult::Stdin => {
            let stdin = std::io::stdin();

            if _mode.contains(&Mode::CountOnly) {
                return stdin.count_search(pattern);
            }

            stdin.normal_search(pattern)?
        }

        ReadResult::File(file) => {
            if _mode.contains(&Mode::CountOnly) {
                return file.count_search(pattern);
            }

            file.normal_search(pattern)?
        }

        ReadResult::MultiFile(multi_file) => {
            if _mode.contains(&Mode::CountOnly) {
                return multi_file.count_search(pattern);
            }

            multi_file.normal_search(pattern)?
        }
    };

    Ok(result)
}

pub trait MatchSearch {
    fn normal_search(&self, pattern: &str) -> Result<MatchResult, Error>;
    fn count_search(&self, pattern: &str) -> Result<MatchResult, Error>;
}

fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

impl MatchSearch for Stdin {
    fn normal_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut match_result: Vec<MatchStdin> = Vec::new();
        let stdin_lock = self.lock();

        for line in stdin_lock.lines() {
            let line = line?;

            if line.contains(pattern) {
                match_result.push(MatchStdin { content: line });
            }
        }

        Ok(MatchResult::Stdin(match_result))
    }

    fn count_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut match_number: usize = 0;
        let stdin_lock = self.lock();

        for line in stdin_lock.lines() {
            if line?.contains(pattern) {
                match_number += 1;
            }
        }

        Ok(MatchResult::Count(match_number))
    }
}

impl MatchSearch for PathBuf {
    fn normal_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut match_result: Vec<MatchFile> = Vec::new();

        let content = open_file(self)?;

        for (line_no, line) in content.lines().enumerate() {
            let line = line?;
            if line.contains(pattern) {
                match_result.push(MatchFile {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(MatchResult::File(match_result))
    }

    fn count_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut match_number: usize = 0;

        let content = open_file(self)?;

        for line in content.lines() {
            if line?.contains(pattern) {
                match_number += 1;
            }
        }

        Ok(MatchResult::Count(match_number))
    }
}

impl MatchSearch for Vec<PathBuf> {
    fn normal_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut dir_match_result: Vec<MatchDir> = Vec::new();

        for single_file in self {
            let mut file_match_result: Vec<MatchFile> = Vec::new();

            let content = open_file(single_file)?;

            for (line_no, line) in content.lines().enumerate() {
                let line = line?;

                if line.contains(pattern) {
                    file_match_result.push(MatchFile {
                        line_no,
                        content: line,
                    });
                }
            }

            if !file_match_result.is_empty() {
                dir_match_result.push(MatchDir {
                    path: single_file.canonicalize()?,
                    file: file_match_result,
                });
            }
        }

        Ok(MatchResult::Dir(dir_match_result))
    }

    fn count_search(&self, pattern: &str) -> Result<MatchResult, Error> {
        let mut dir_match_number: usize = 0;

        for single_file in self {
            let mut file_match_number: usize = 0;

            let content = open_file(single_file)?;

            for line in content.lines() {
                if line?.contains(pattern) {
                    file_match_number += 1;
                }
            }

            dir_match_number += file_match_number;
        }

        Ok(MatchResult::Count(dir_match_number))
    }
}
