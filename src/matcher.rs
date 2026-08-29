use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    cli::MatchOptions,
    error::Error,
    printer::{
        CountResult, Match, NormalResult, Range,
        SearchResult::{self},
    },
    reader::ReadResult,
};

pub fn search(
    pattern: &str,
    read_result: &ReadResult,
    mode: &MatchOptions,
) -> Result<SearchResult, Error> {
    Ok(match read_result {
        ReadResult::Stdin => matcher(pattern, io::stdin().lock(), None, mode)?,

        ReadResult::File(file) => {
            let reader = BufReader::new(File::open(file)?);
            matcher(pattern, reader, Some(file.clone()), mode)?
        }

        ReadResult::MultiFile(files) => search_files(files, pattern, mode)?,
    })
}

fn matcher<W: BufRead>(
    pattern: &str,
    reader: W,
    path: Option<PathBuf>,
    mode: &MatchOptions,
) -> Result<SearchResult, Error> {
    match mode {
        MatchOptions::Normal => {
            let normal = normal(reader, path, pattern)?;
            Ok(SearchResult::Normal(vec![normal]))
        }
        MatchOptions::CountOnly => {
            let count = count(reader, path, pattern)?;
            Ok(SearchResult::Count(vec![count]))
        }
        MatchOptions::IgnoreCase => {
            let ignore = ignore_case(reader, path, pattern)?;
            Ok(SearchResult::Normal(vec![ignore]))
        }
        MatchOptions::IgnoreAndCount => {
            let ignore_count = ignore_count(reader, path, pattern)?;
            Ok(SearchResult::Count(vec![ignore_count]))
        }
    }
}

fn search_files(
    files: &[PathBuf],
    pattern: &str,
    mode: &MatchOptions,
) -> Result<SearchResult, Error> {
    let mut normal_results = Vec::new();
    let mut count_results = Vec::new();

    for file in files {
        let reader = BufReader::new(File::open(file)?);

        match matcher(pattern, reader, Some(file.clone()), mode)? {
            SearchResult::Normal(mut result) => {
                if !result[0].matches.is_empty() {
                    normal_results.append(&mut result);
                }
            }
            SearchResult::Count(mut result) => {
                if result[0].number != 0 {
                    count_results.append(&mut result);
                }
            }
        }
    }

    match mode {
        MatchOptions::Normal | MatchOptions::IgnoreCase => Ok(SearchResult::Normal(normal_results)),
        MatchOptions::CountOnly | MatchOptions::IgnoreAndCount => {
            Ok(SearchResult::Count(count_results))
        }
    }
}

fn normal<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
) -> Result<NormalResult, Error> {
    let mut matches: Vec<Match> = Vec::new();
    for (line_no, line) in reader.lines().enumerate() {
        let line = line?;

        if let Some(index) = line.find(pattern) {
            matches.push(Match {
                line_num: line_no,
                content: line,
                range: Range {
                    start: index,
                    end: index + pattern.len(),
                },
            });
        }
    }
    Ok(NormalResult { path, matches })
}

fn count<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
) -> Result<CountResult, Error> {
    let mut count: usize = 0;
    for line in reader.lines() {
        let line = line?;

        if line.contains(pattern) {
            count += 1;
        }
    }
    Ok(CountResult {
        path,
        number: count,
    })
}

fn ignore_case<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
) -> Result<NormalResult, Error> {
    let mut matches: Vec<Match> = Vec::new();
    let pattern = &pattern.to_lowercase();

    for (line_no, line) in reader.lines().enumerate() {
        let line = line?;

        if let Some(index) = line.to_lowercase().find(pattern) {
            matches.push(Match {
                line_num: line_no,
                content: line,
                range: Range {
                    start: index,
                    end: index + pattern.len(),
                },
            });
        }
    }
    Ok(NormalResult { path, matches })
}

fn ignore_count<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
) -> Result<CountResult, Error> {
    let mut count: usize = 0;
    let pattern = &pattern.to_lowercase();
    for line in reader.lines() {
        let line = line?;

        if line.to_lowercase().contains(pattern) {
            count += 1;
        }
    }
    Ok(CountResult {
        path,
        number: count,
    })
}
