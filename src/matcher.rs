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

// pub fn search(
//     pattern: &str,
//     read_result: &Input,
//     mode: &MatchOptions,
// ) -> Result<SearchResult, Error> {
//     Ok(match read_result {
//         Input::Stdin => matcher(pattern, io::stdin().lock(), None, mode)?,

//         Input::MultiFile(files) => search_files(files, pattern, mode)?,
//     })
// }

// fn matcher<W: BufRead>(
//     pattern: &str,
//     reader: W,
//     path: Option<PathBuf>,
//     mode: &MatchOptions,
// ) -> Result<SearchResult, Error> {
//     match mode {
//         MatchOptions::Normal => {
//             let normal = normal(reader, path, pattern)?;
//             Ok(SearchResult::Normal(vec![normal]))
//         }
//         MatchOptions::CountOnly => {
//             let count = count(reader, path, pattern)?;
//             Ok(SearchResult::Count(vec![count]))
//         }
//         MatchOptions::IgnoreCase => {
//             let ignore = ignore_case(reader, path, pattern)?;
//             Ok(SearchResult::Normal(vec![ignore]))
//         }
//         MatchOptions::IgnoreAndCount => {
//             let ignore_count = ignore_count(reader, path, pattern)?;
//             Ok(SearchResult::Count(vec![ignore_count]))
//         }
//     }
// }

// fn search_files(
//     files: &[PathBuf],
//     pattern: &str,
//     mode: &MatchOptions,
// ) -> Result<SearchResult, Error> {
//     let mut normal_results = Vec::new();
//     let mut count_results = Vec::new();

//     for file in files {
//         let reader = BufReader::new(File::open(file)?);

//         match matcher(pattern, reader, Some(file.clone()), mode)? {
//             SearchResult::Normal(mut result) => {
//                 if !result[0].matches.is_empty() {
//                     normal_results.append(&mut result);
//                 }
//             }
//             SearchResult::Count(mut result) => {
//                 if result[0].number != 0 {
//                     count_results.append(&mut result);
//                 }
//             }
//         }
//     }

//     match mode {
//         MatchOptions::Normal | MatchOptions::IgnoreCase => Ok(SearchResult::Normal(normal_results)),
//         MatchOptions::CountOnly | MatchOptions::IgnoreAndCount => {
//             Ok(SearchResult::Count(count_results))
//         }
//     }
// }

// fn normal<R: BufRead>(
//     reader: R,
//     path: Option<PathBuf>,
//     pattern: &str,
// ) -> Result<NormalResult, Error> {
//     let mut matches: Vec<Match> = Vec::new();
//     for (line_no, line) in reader.lines().enumerate() {
//         // appear error if encounter binary file
//         let line = line?;

//         if let Some(index) = line.find(pattern) {
//             matches.push(Match {
//                 line_num: line_no,
//                 content: line,
//                 range: Range {
//                     start: index,
//                     end: index + pattern.len(),
//                 },
//             });
//         }
//     }
//     Ok(NormalResult { path, matches })
// }

// fn count<R: BufRead>(
//     reader: R,
//     path: Option<PathBuf>,
//     pattern: &str,
// ) -> Result<CountResult, Error> {
//     let mut count: usize = 0;
//     for line in reader.lines() {
//         let line = line?;

//         if line.contains(pattern) {
//             count += 1;
//         }
//     }
//     Ok(CountResult {
//         path,
//         number: count,
//     })
// }

// fn ignore_case<R: BufRead>(
//     reader: R,
//     path: Option<PathBuf>,
//     pattern: &str,
// ) -> Result<NormalResult, Error> {
//     let mut matches: Vec<Match> = Vec::new();
//     let pattern = &pattern.to_lowercase();

//     for (line_no, line) in reader.lines().enumerate() {
//         let line = line?;

//         if let Some(index) = line.to_lowercase().find(pattern) {
//             matches.push(Match {
//                 line_num: line_no,
//                 content: line,
//                 range: Range {
//                     start: index,
//                     end: index + pattern.len(),
//                 },
//             });
//         }
//     }
//     Ok(NormalResult { path, matches })
// }

// fn ignore_count<R: BufRead>(
//     reader: R,
//     path: Option<PathBuf>,
//     pattern: &str,
// ) -> Result<CountResult, Error> {
//     let mut count: usize = 0;
//     let pattern = &pattern.to_lowercase();
//     for line in reader.lines() {
//         let line = line?;

//         if line.to_lowercase().contains(pattern) {
//             count += 1;
//         }
//     }
//     Ok(CountResult {
//         path,
//         number: count,
//     })
// }

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
        // Ok(Box::new(count))
        Ok(MatchResult::Count(count))
    } else {
        let normal = NormalResult {
            path: path.clone(),
            matches,
        };
        // Ok(Box::new(normal))
        Ok(MatchResult::Normal(normal))
    }
}

// pub trait Matched: Debug {}

// impl Matched for NormalResult {}

// impl Matched for CountResult {}

fn matchers(pattern: &str, files: &[PathBuf], mode: &MatchMode) -> Result<SearchResult, Error> {
    // let mut result: Vec<Box<dyn Matched>> = Vec::new();
    // let mut result: SearchResult = Vec::new();
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
    // Ok(result)
    if !normals.is_empty() {
        Ok(SearchResult::Normal(normals))
    } else {
        Ok(SearchResult::Count(counts))
    }
}

pub fn search(pattern: &str, read_result: &Input, mode: &MatchMode) -> Result<SearchResult, Error> {
    // Ok(match read_result {
    //     Input::Stdin => {
    //         let result = _new_match(pattern, io::stdin().lock(), None, mode)?;
    //         return Ok(vec![result]);
    //     }
    //     Input::MultiFile(files) => {
    //         let result = new_match_files(pattern, files, mode)?;
    //         return Ok(result);
    //     }
    // })
    // let result: SearchResult =
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

    // Ok(result)
}

#[derive(Debug)]
pub enum MatchResult {
    Normal(NormalResult),
    Count(CountResult),
}
