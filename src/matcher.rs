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

pub fn new_search(
    pattern: &str,
    read_result: &ReadResult,
    mode: &MatchOptions,
) -> Result<SearchResult, Error> {
    let search_result = match read_result {
        ReadResult::Stdin => {
            let stdin = io::stdin();
            let stdin_lock = stdin.lock();

            match mode {
                MatchOptions::Normal => {
                    let normal = normal(stdin_lock, None, pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
                MatchOptions::CountOnly => {
                    let count = count(stdin_lock, None, pattern, mode)?;
                    SearchResult::Count(vec![count])
                }
                MatchOptions::IgnoreCase => {
                    let normal = ignore_case(stdin_lock, None, pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
                _ => return Err(Error::UnFinished),
            }
        }

        ReadResult::File(file) => {
            let open_file = File::open(file)?;
            let reader = BufReader::new(open_file);

            match mode {
                MatchOptions::Normal => {
                    let normal = normal(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
                MatchOptions::CountOnly => {
                    let count = count(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Count(vec![count])
                }
                MatchOptions::IgnoreCase => {
                    let normal = ignore_case(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }

                _ => return Err(Error::UnFinished),
            }
        }

        ReadResult::MultiFile(_dir) => {
            let mut dir_normal = Vec::new();
            let mut dir_count = Vec::new();
            for file in _dir {
                let open_file = File::open(file)?;
                let reader = BufReader::new(open_file);

                match mode {
                    MatchOptions::Normal => {
                        let normal = normal(reader, Some(file.clone()), pattern, mode)?;
                        if !normal.matches.is_empty() {
                            dir_normal.push(normal);
                        }
                    }
                    MatchOptions::CountOnly => {
                        let count = count(reader, Some(file.clone()), pattern, mode)?;
                        if count.number != 0 {
                            dir_count.push(count);
                        }
                    }
                    MatchOptions::IgnoreCase => {
                        let normal = ignore_case(reader, Some(file.clone()), pattern, mode)?;
                        if !normal.matches.is_empty() {
                            dir_normal.push(normal);
                        }
                    }

                    _ => return Err(Error::UnFinished),
                }
            }

            if !dir_count.is_empty() {
                SearchResult::Count(dir_count)
            } else {
                SearchResult::Normal(dir_normal)
            }
        }
    };

    Ok(search_result)
}

fn normal<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
    _mode: &MatchOptions,
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
    _mode: &MatchOptions,
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
    _mode: &MatchOptions,
) -> Result<NormalResult, Error> {
    let mut matches: Vec<Match> = Vec::new();
    for (line_no, line) in reader.lines().enumerate() {
        let line = line?;

        if let Some(index) = line.to_lowercase().find(&pattern.to_lowercase()) {
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
