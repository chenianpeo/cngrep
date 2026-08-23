use std::{
    fs::File,
    io::{BufRead, BufReader, Stdin},
    path::PathBuf,
};

use crate::{
    cli::MatchOptions,
    error::Error,
    printer::{
        CountMultiFile, CountResult, IgnoreCaseResult, MatchMultiFile, MatchStdinFile,
        NormalResult::{self},
        SearchResult,
    },
    reader::ReadResult,
};

pub fn search(
    pattern: &str,
    read_result: &ReadResult,
    mode: &[MatchOptions],
) -> Result<SearchResult, Error> {
    let result: SearchResult = match read_result {
        ReadResult::Stdin => {
            let stdin = std::io::stdin();

            if mode.contains(&MatchOptions::CountOnly) {
                return stdin.count_search(pattern);
            }

            if mode.contains(&MatchOptions::IgnoreCase) {
                return stdin.ignore_case_search(pattern);
            }

            stdin.normal_search(pattern)?
        }

        ReadResult::File(file) => {
            if mode.contains(&MatchOptions::CountOnly) {
                return file.count_search(pattern);
            }

            if mode.contains(&MatchOptions::IgnoreCase) {
                return file.ignore_case_search(pattern);
            }

            file.normal_search(pattern)?
        }

        ReadResult::MultiFile(multi_file) => {
            if mode.contains(&MatchOptions::CountOnly) {
                return multi_file.count_search(pattern);
            }

            if mode.contains(&MatchOptions::IgnoreCase) {
                return multi_file.ignore_case_search(pattern);
            }

            multi_file.normal_search(pattern)?
        }
    };

    Ok(result)
}

pub trait MatchSearch {
    fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error>;
    fn count_search(&self, pattern: &str) -> Result<SearchResult, Error>;
    fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error>;
}

fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

impl MatchSearch for Stdin {
    fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_result: Vec<MatchStdinFile> = Vec::new();
        let stdin_lock = self.lock();

        for (line_no, line) in stdin_lock.lines().enumerate() {
            let line = line?;

            if line.contains(pattern) {
                match_result.push(MatchStdinFile {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(SearchResult::Normal(NormalResult::StdinFile(match_result)))
    }

    fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_number: usize = 0;
        let stdin_lock = self.lock();

        for line in stdin_lock.lines() {
            if line?.contains(pattern) {
                match_number += 1;
            }
        }

        Ok(SearchResult::Count(CountResult::StdinFile(match_number)))
    }

    fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_result: Vec<MatchStdinFile> = Vec::new();
        let stdin_lock = self.lock();

        for (line_no, line) in stdin_lock.lines().enumerate() {
            let line = line?;

            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                match_result.push(MatchStdinFile {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(SearchResult::IgnoreCase(IgnoreCaseResult::StdinFile(
            match_result,
        )))
    }
}

impl MatchSearch for PathBuf {
    fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_result: Vec<MatchStdinFile> = Vec::new();

        let content = open_file(self)?;

        for (line_no, line) in content.lines().enumerate() {
            let line = line?;
            if line.contains(pattern) {
                match_result.push(MatchStdinFile {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(SearchResult::Normal(NormalResult::StdinFile(match_result)))
    }

    fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_number: usize = 0;

        let content = open_file(self)?;

        for line in content.lines() {
            if line?.contains(pattern) {
                match_number += 1;
            }
        }

        Ok(SearchResult::Count(CountResult::StdinFile(match_number)))
    }

    fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut match_result: Vec<MatchStdinFile> = Vec::new();

        let content = open_file(self)?;

        for (line_no, line) in content.lines().enumerate() {
            let line = line?;

            if line.to_lowercase().contains(&pattern.to_lowercase()) {
                match_result.push(MatchStdinFile {
                    line_no,
                    content: line,
                });
            }
        }

        Ok(SearchResult::IgnoreCase(IgnoreCaseResult::StdinFile(
            match_result,
        )))
    }
}

impl MatchSearch for Vec<PathBuf> {
    fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut dir_match_result: Vec<MatchMultiFile> = Vec::new();

        for single_file in self {
            let mut file_match_result: Vec<MatchStdinFile> = Vec::new();

            let content = open_file(single_file)?;

            for (line_no, line) in content.lines().enumerate() {
                let line = line?;

                if line.contains(pattern) {
                    file_match_result.push(MatchStdinFile {
                        line_no,
                        content: line,
                    })
                }
            }

            if !file_match_result.is_empty() {
                dir_match_result.push(MatchMultiFile {
                    path: single_file.canonicalize()?,
                    file: file_match_result,
                });
            }
        }

        Ok(SearchResult::Normal(NormalResult::MultiFile(
            dir_match_result,
        )))
    }

    fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut dir_match: Vec<CountMultiFile> = Vec::new();

        for single_file in self {
            let mut file_match_number: usize = 0;

            let content = open_file(single_file)?;

            for line in content.lines() {
                if line?.contains(pattern) {
                    file_match_number += 1;
                }
            }

            dir_match.push(CountMultiFile {
                path: single_file.to_path_buf(),
                number: file_match_number,
            });
        }

        Ok(SearchResult::Count(CountResult::MultiFile(dir_match)))
    }

    fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
        let mut dir_match_result: Vec<MatchMultiFile> = Vec::new();

        for single_file in self {
            let mut file_match_result: Vec<MatchStdinFile> = Vec::new();

            let content = open_file(single_file)?;

            for (line_no, line) in content.lines().enumerate() {
                let line = line?;

                if line.to_lowercase().contains(&pattern.to_lowercase()) {
                    file_match_result.push(MatchStdinFile {
                        line_no,
                        content: line,
                    });
                }
            }

            if !file_match_result.is_empty() {
                dir_match_result.push(MatchMultiFile {
                    path: single_file.canonicalize()?,
                    file: file_match_result,
                });
            }
        }

        Ok(SearchResult::IgnoreCase(IgnoreCaseResult::MultiFile(
            dir_match_result,
        )))
    }
}
