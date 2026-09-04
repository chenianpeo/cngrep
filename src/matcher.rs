use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    vec,
};

use crate::{
    cli::MatchMode,
    common::Range,
    error::Error,
    printer::{
        CountResult, Match, NormalResult,
        SearchResult::{self},
    },
    reader::Input,
};

#[derive(Debug)]
pub enum MatchResult {
    Normal(NormalResult),
    Count(CountResult),
}

pub fn search(pattern: &str, read_result: &Input, mode: &MatchMode) -> Result<SearchResult, Error> {
    match read_result {
        Input::Stdin => {
            let result = matcher(pattern, io::stdin().lock(), None, mode)?;

            match result {
                MatchResult::Normal(normal) => Ok(SearchResult::Normal(vec![normal])),
                MatchResult::Count(count) => Ok(SearchResult::Count(vec![count])),
            }
        }
        Input::MultiFile(files) => {
            let result = matchers(pattern, files, mode)?;
            Ok(result)
        }
    }
}

fn matcher<W: BufRead>(
    pattern: &str,
    reader: W,
    path: Option<PathBuf>,
    mode: &MatchMode,
) -> Result<MatchResult, Error> {
    let pattern_len = pattern.len();
    let pattern_lower = pattern.to_lowercase();

    let mut matches: Vec<Match> = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line = line?;

        let (matched, start) = if mode.ignore_case {
            let line_lower = line.to_lowercase();

            match line_lower.find(&pattern_lower) {
                Some(index) => (true, index),
                None => (false, 0),
            }
        } else {
            match line.find(pattern) {
                Some(index) => (true, index),
                None => (false, 0),
            }
        };

        if matched {
            let end = start + pattern_len;

            matches.push(Match {
                line_num,
                content: line,
                range: Range { start, end },
            });
        }
    }

    let number = matches.len();

    if mode.count {
        let count = CountResult { path, number };
        Ok(MatchResult::Count(count))
    } else {
        let normal = NormalResult {
            path: path.clone(),
            matches,
        };
        Ok(MatchResult::Normal(normal))
    }
}

fn matchers(pattern: &str, files: &[PathBuf], mode: &MatchMode) -> Result<SearchResult, Error> {
    let mut normals: Vec<NormalResult> = Vec::new();
    let mut counts: Vec<CountResult> = Vec::new();

    for file in files {
        let reader = BufReader::new(File::open(file)?);

        let matcher = matcher(pattern, reader, Some(file.to_path_buf()), mode)?;

        match matcher {
            MatchResult::Normal(normal) => {
                if normal.matches.is_empty() {
                    continue;
                }
                normals.push(normal)
            }
            MatchResult::Count(count) => {
                if count.number == 0 {
                    continue;
                }
                counts.push(count)
            }
        }
    }

    if !normals.is_empty() {
        Ok(SearchResult::Normal(normals))
    } else {
        Ok(SearchResult::Count(counts))
    }
}
