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

// pub fn search(
//     pattern: &str,
//     read_result: &ReadResult,
//     mode: &[MatchOptions],
// ) -> Result<SearchResult, Error> {
//     let result: SearchResult = match read_result {
//         ReadResult::Stdin => {
//             let stdin = std::io::stdin();

//             if mode.contains(&MatchOptions::CountOnly) {
//                 return stdin.count_search(pattern);
//             }

//             if mode.contains(&MatchOptions::IgnoreCase) {
//                 return stdin.ignore_case_search(pattern);
//             }

//             stdin.normal_search(pattern)?
//         }

//         ReadResult::File(file) => {
//             if mode.contains(&MatchOptions::CountOnly) {
//                 return file.count_search(pattern);
//             }

//             if mode.contains(&MatchOptions::IgnoreCase) {
//                 return file.ignore_case_search(pattern);
//             }

//             file.normal_search(pattern)?
//         }

//         ReadResult::MultiFile(multi_file) => {
//             if mode.contains(&MatchOptions::CountOnly) {
//                 return multi_file.count_search(pattern);
//             }

//             if mode.contains(&MatchOptions::IgnoreCase) {
//                 return multi_file.ignore_case_search(pattern);
//             }

//             multi_file.normal_search(pattern)?
//         }
//     };

//     Ok(result)
// }

// pub trait MatchSearch {
//     fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error>;
//     fn count_search(&self, pattern: &str) -> Result<SearchResult, Error>;
//     fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error>;
// }

// fn open_file(path: &PathBuf) -> Result<BufReader<File>, Error> {
//     let file = File::open(path)?;
//     Ok(BufReader::new(file))
// }

// // TODO the logic is extremely complex
// impl MatchSearch for Stdin {
//     fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let stdin_lock = self.lock();

//         let mut new_match_result: Vec<Match> = Vec::new();

//         for (line_no, line) in stdin_lock.lines().enumerate() {
//             let line = line?;
//             let mut index_start = 0;

//             if line.contains(pattern) {
//                 if let Some(index) = line.find(pattern) {
//                     index_start = index;
//                 }
//                 let index_end = index_start + pattern.len();
//                 let match_range = Range {
//                     start: index_start,
//                     end: index_end,
//                 };

//                 new_match_result.push(Match {
//                     line_num: line_no,
//                     content: line,
//                     range: match_range,
//                 });
//             }
//         }

//         let new_result = NormalResult {
//             path: None,
//             matches: new_match_result,
//         };
//         let _result = SearchResult::Normal(vec![new_result]);

//         Ok(_result)
//     }

//     // TODO count only and ignore case logic have problem
//     // should let many option in a search space
//     fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut match_number: usize = 0;
//         let stdin_lock = self.lock();

//         for line in stdin_lock.lines() {
//             if line?.contains(pattern) {
//                 match_number += 1;
//             }
//         }

//         let new_result = CountResult {
//             path: None,
//             number: match_number,
//         };
//         let _result = SearchResult::Count(vec![new_result]);

//         Ok(_result)
//     }

//     fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let stdin_lock = self.lock();
//         let mut new_match_result: Vec<Match> = Vec::new();

//         for (line_no, line) in stdin_lock.lines().enumerate() {
//             let line = line?;
//             let mut index_start = 0;

//             if line.to_lowercase().contains(&pattern.to_lowercase()) {
//                 if let Some(index) = line.to_lowercase().find(pattern) {
//                     index_start = index;
//                 }

//                 let index_end = index_start + pattern.len();
//                 let match_range = Range {
//                     start: index_start,
//                     end: index_end,
//                 };

//                 new_match_result.push(Match {
//                     line_num: line_no,
//                     content: line,
//                     range: match_range,
//                 });
//             }
//         }

//         let new_result = SearchResult::Normal(vec![NormalResult {
//             path: None,
//             matches: new_match_result,
//         }]);

//         Ok(new_result)
//     }
// }

// impl MatchSearch for PathBuf {
//     fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut new_match_result: Vec<Match> = Vec::new();

//         let content = open_file(self)?;

//         for (line_no, line) in content.lines().enumerate() {
//             let line = line?;
//             let mut index_start = 0;
//             if line.contains(pattern) {
//                 if let Some(index) = line.find(pattern) {
//                     index_start = index;
//                 }
//                 let index_end = index_start + pattern.len();
//                 let match_range = Range {
//                     start: index_start,
//                     end: index_end,
//                 };

//                 new_match_result.push(Match {
//                     line_num: line_no,
//                     content: line,
//                     range: match_range,
//                 });
//             }
//         }

//         Ok(SearchResult::Normal(vec![NormalResult {
//             path: Some(self.to_path_buf()),
//             matches: new_match_result,
//         }]))
//     }

//     fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut match_number: usize = 0;

//         let content = open_file(self)?;

//         for line in content.lines() {
//             if line?.contains(pattern) {
//                 match_number += 1;
//             }
//         }

//         Ok(SearchResult::Count(vec![CountResult {
//             path: Some(self.to_path_buf()),
//             number: match_number,
//         }]))
//     }

//     fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut new_match_result: Vec<Match> = Vec::new();

//         let content = open_file(self)?;

//         for (line_no, line) in content.lines().enumerate() {
//             let line = line?;
//             let mut index_start = 0;

//             if line.to_lowercase().contains(&pattern.to_lowercase()) {
//                 if let Some(index) = line.to_lowercase().find(pattern) {
//                     index_start = index;
//                 }
//                 let index_end = index_start + pattern.len();
//                 let match_range = Range {
//                     start: index_start,
//                     end: index_end,
//                 };
//                 new_match_result.push(Match {
//                     line_num: line_no,
//                     content: line,
//                     range: match_range,
//                 });
//             }
//         }

//         Ok(SearchResult::Normal(vec![NormalResult {
//             path: Some(self.to_path_buf()),
//             matches: new_match_result,
//         }]))
//     }
// }

// impl MatchSearch for Vec<PathBuf> {
//     fn normal_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut new_result: Vec<NormalResult> = Vec::new();

//         for single_file in self {
//             let mut new_match_result: Vec<Match> = Vec::new();

//             let content = open_file(single_file)?;

//             for (line_no, line) in content.lines().enumerate() {
//                 let line = line?;
//                 let mut index_start = 0;

//                 if line.contains(pattern) {
//                     if let Some(index) = line.find(pattern) {
//                         index_start = index;
//                     }
//                     let index_end = index_start + pattern.len();
//                     let match_range = Range {
//                         start: index_start,
//                         end: index_end,
//                     };

//                     new_match_result.push(Match {
//                         line_num: line_no,
//                         content: line,
//                         range: match_range,
//                     });
//                 }
//             }

//             if !new_match_result.is_empty() {
//                 new_result.push(NormalResult {
//                     path: Some(single_file.to_path_buf()),
//                     matches: new_match_result,
//                 });
//             }
//         }

//         Ok(SearchResult::Normal(new_result))
//     }

//     fn count_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut new_result: Vec<CountResult> = Vec::new();

//         for single_file in self {
//             let mut file_match_number: usize = 0;

//             let content = open_file(single_file)?;

//             for line in content.lines() {
//                 if line?.contains(pattern) {
//                     file_match_number += 1;
//                 }
//             }

//             if file_match_number != 0 {
//                 new_result.push(CountResult {
//                     path: Some(single_file.to_path_buf()),
//                     number: file_match_number,
//                 });
//             }
//         }

//         Ok(SearchResult::Count(new_result))
//     }

//     fn ignore_case_search(&self, pattern: &str) -> Result<SearchResult, Error> {
//         let mut new_result: Vec<NormalResult> = Vec::new();

//         for single_file in self {
//             let mut new_match_result: Vec<Match> = Vec::new();

//             let content = open_file(single_file)?;

//             for (line_no, line) in content.lines().enumerate() {
//                 let line = line?;
//                 let mut index_start = 0;

//                 if line.to_lowercase().contains(&pattern.to_lowercase()) {
//                     if let Some(index) = line.to_lowercase().find(pattern) {
//                         index_start = index;
//                     }

//                     let index_end = index_start + pattern.len();
//                     let match_range = Range {
//                         start: index_start,
//                         end: index_end,
//                     };
//                     new_match_result.push(Match {
//                         line_num: line_no,
//                         content: line,
//                         range: match_range,
//                     });
//                 }
//             }

//             if !new_match_result.is_empty() {
//                 new_result.push(NormalResult {
//                     path: Some(single_file.to_path_buf()),
//                     matches: new_match_result,
//                 });
//             }
//         }

//         Ok(SearchResult::Normal(new_result))
//     }
// }

pub fn new_search(
    pattern: &str,
    read_result: &ReadResult,
    mode: &MatchOptions,
) -> Result<SearchResult, Error> {
    let search_result = match read_result {
        ReadResult::Stdin => {
            let stdin = io::stdin();
            let stdin_lock = stdin.lock();

            // let normal = new_normal(stdin_lock, None, pattern, mode)?;
            // SearchResult::Normal(vec![normal])
            match mode {
                MatchOptions::Normal => {
                    let normal = new_normal(stdin_lock, None, pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
                MatchOptions::CountOnly => {
                    let count = new_count(stdin_lock, None, pattern, mode)?;
                    SearchResult::Count(vec![count])
                }
                MatchOptions::IgnoreCase => {
                    let normal = new_ignore(stdin_lock, None, pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
            }
        }

        ReadResult::File(file) => {
            let open_file = File::open(file)?;
            let reader = BufReader::new(open_file);

            // let normal = new_normal(reader, Some(file.clone()), pattern, mode)?;
            // SearchResult::Normal(vec![normal])
            match mode {
                MatchOptions::Normal => {
                    let normal = new_normal(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
                MatchOptions::CountOnly => {
                    let count = new_count(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Count(vec![count])
                }
                MatchOptions::IgnoreCase => {
                    let normal = new_ignore(reader, Some(file.clone()), pattern, mode)?;
                    SearchResult::Normal(vec![normal])
                }
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
                        let normal = new_normal(reader, Some(file.clone()), pattern, mode)?;
                        if !normal.matches.is_empty() {
                            dir_normal.push(normal);
                        }
                    }
                    MatchOptions::CountOnly => {
                        let count = new_count(reader, Some(file.clone()), pattern, mode)?;
                        if count.number != 0 {
                            dir_count.push(count);
                        }
                    }
                    MatchOptions::IgnoreCase => {
                        let normal = new_ignore(reader, Some(file.clone()), pattern, mode)?;
                        if !normal.matches.is_empty() {
                            dir_normal.push(normal);
                        }
                    }
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

// fn new_match<R: BufRead>(
//     reader: R,
//     path: Option<PathBuf>,
//     pattern: &str,
//     mode: &MatchOptions,
// ) -> Result<SearchResult, Error> {
//     match mode {
//         MatchOptions::Normal => {
//             let mut matches: Vec<Match> = Vec::new();
//             for (line_no, line) in reader.lines().enumerate() {
//                 let line = line?;

//                 if let Some(index) = line.find(pattern) {
//                     matches.push(Match {
//                         line_num: line_no,
//                         content: line,
//                         range: Range {
//                             start: index,
//                             end: index + pattern.len(),
//                         },
//                     });
//                 }
//             }

//             Ok(SearchResult::Normal(vec![NormalResult { path, matches }]))
//         }

//         _ => Err(Error::NotFound("not found".into())),
//     }
// }

fn new_normal<R: BufRead>(
    reader: R,
    path: Option<PathBuf>,
    pattern: &str,
    _mode: &MatchOptions,
) -> Result<NormalResult, Error> {
    // match _mode {
    //     MatchOptions::Normal => {
    //         let mut matches: Vec<Match> = Vec::new();
    //         for (line_no, line) in reader.lines().enumerate() {
    //             let line = line?;

    //             if let Some(index) = line.find(pattern) {
    //                 matches.push(Match {
    //                     line_num: line_no,
    //                     content: line,
    //                     range: Range {
    //                         start: index,
    //                         end: index + pattern.len(),
    //                     },
    //                 });
    //             }
    //         }
    //         Ok(NormalResult { path, matches })
    //     }

    //     MatchOptions::IgnoreCase => {
    //         let mut matches: Vec<Match> = Vec::new();
    //         for (line_no, line) in reader.lines().enumerate() {
    //             let line = line?;

    //             if let Some(index) = line.to_lowercase().find(&pattern.to_lowercase()) {
    //                 matches.push(Match {
    //                     line_num: line_no,
    //                     content: line,
    //                     range: Range {
    //                         start: index,
    //                         end: index + pattern.len(),
    //                     },
    //                 });
    //             }
    //         }
    //         Ok(NormalResult { path, matches })
    //     }

    //     _ => Err(Error::UnFinished),
    // }
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

fn new_count<R: BufRead>(
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

fn new_ignore<R: BufRead>(
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
