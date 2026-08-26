use std::fmt::Display;
use std::path::PathBuf;

use crate::error::Error;

#[derive(Debug)]
pub enum OutputPosition {
    Terminal,
    File(PathBuf),
}

// check if the search result is empty and return Not Found
// todo: should move to matching stage
// fn is_matched<T>(r: &[T]) -> Result<(), Error> {
//     let not_fount = "Not Found".red();

//     if r.is_empty() {
//         return Err(Error::NotFound(not_fount));
//     }

//     Ok(())
// }

// color output content
pub trait Color: Display {
    fn color(&self, code: u8) -> String {
        format!("\x1b[{}m{}\x1b[0m", code, self)
    }

    fn red(&self) -> String {
        self.color(31)
    }

    fn green(&self) -> String {
        self.color(32)
    }

    fn yellow(&self) -> String {
        self.color(33)
    }

    fn blue(&self) -> String {
        self.color(34)
    }
}

impl<T: Display> Color for T {}

// output result
// pub fn render(pattern: &str, result: &SearchResult, mode: &[OutputOptions]) -> Result<(), Error> {
//     let mut output_position = OutputPosition::Terminal;
//     for options in mode {
//         output_position = match options {
//             OutputOptions::OutputFile(path) => OutputPosition::File(path.clone()),
//             _ => OutputPosition::Terminal,
//         }
//     }

//     match output_position {
//         OutputPosition::File(path) => {
//             let mut output_file = File::create(path)?;

//             match result {
//                 SearchResult::Normal(normal_result) => match normal_result {
//                     NormalResult::StdinFile(file_result) => {
//                         is_matched(file_result)?;

//                         for file in file_result {
//                             writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
//                         }
//                     }

//                     NormalResult::MultiFile(dir_result) => {
//                         is_matched(dir_result)?;

//                         for (dir_no, dir) in dir_result.iter().enumerate() {
//                             writeln!(output_file, "{}", dir.path.display())?;

//                             for file in dir.file.iter() {
//                                 writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
//                             }

//                             if dir_no != dir_result.len() - 1 {
//                                 writeln!(output_file)?;
//                             }
//                         }
//                     }
//                 },

//                 SearchResult::Count(count_result) => match count_result {
//                     CountResult::StdinFile(stdin_file) => {
//                         if stdin_file == &0 {
//                             let not_fount = "Not Found".red();
//                             return Err(Error::NotFound(not_fount));
//                         } else {
//                             writeln!(output_file, "{stdin_file}")?;
//                         }
//                     }

//                     CountResult::MultiFile(multi_file) => {
//                         let mut total_number: usize = 0;

//                         for single_file in multi_file {
//                             if single_file.number != 0 {
//                                 total_number += single_file.number;
//                                 writeln!(
//                                     output_file,
//                                     "{}:{}",
//                                     single_file.path.display(),
//                                     single_file.number
//                                 )?;
//                             }
//                         }

//                         if total_number == 0 {
//                             let not_fount = "Not Found".red();
//                             return Err(Error::NotFound(not_fount));
//                         }

//                         writeln!(output_file, "Total Match Number: {total_number}")?;
//                     }
//                 },
//             }
//         }

//         OutputPosition::Terminal => match result {
//             SearchResult::Normal(normal_result) => match normal_result {
//                 NormalResult::StdinFile(file_result) => {
//                     is_matched(file_result)?;
//                     let pattern_lowercase = pattern.to_lowercase();
//                     let pattern_len = pattern.len();

//                     for file in file_result {
//                         let file_content = file.content.clone();
//                         let mut index_pattern = 0;

//                         if let Some(index) = file_content.to_lowercase().find(&pattern_lowercase) {
//                             index_pattern = index;
//                         }

//                         let match_content =
//                             &file_content[index_pattern..(index_pattern + pattern_len)];

//                         println!(
//                             "{}:{}",
//                             (file.line_no + 1).blue(),
//                             file.content.replace(match_content, &match_content.green())
//                         );
//                     }
//                 }

//                 NormalResult::MultiFile(dir_result) => {
//                     is_matched(dir_result)?;
//                     let pattern_lowercase = pattern.to_lowercase();
//                     let pattern_len = pattern.len();

//                     for (dir_no, dir) in dir_result.iter().enumerate() {
//                         println!("{}", dir.path.display().yellow());

//                         for file in dir.file.iter() {
//                             let file_content = file.content.clone();
//                             let mut index_pattern = 0;

//                             if let Some(index) =
//                                 file_content.to_lowercase().find(&pattern_lowercase)
//                             {
//                                 index_pattern = index;
//                             }
//                             let match_content =
//                                 &file_content[index_pattern..(index_pattern + pattern_len)];

//                             println!(
//                                 "{}:{}",
//                                 (file.line_no + 1).blue(),
//                                 file.content.replace(match_content, &match_content.green())
//                             );
//                         }

//                         if dir_no != dir_result.len() - 1 {
//                             println!();
//                         }
//                     }
//                 }
//             },

//             SearchResult::Count(count_result) => match count_result {
//                 CountResult::StdinFile(stdin_file) => {
//                     if stdin_file == &0 {
//                         let not_fount = "Not Found".red();
//                         return Err(Error::NotFound(not_fount));
//                     }
//                     println!("{stdin_file}")
//                 }

//                 CountResult::MultiFile(multi_file) => {
//                     let mut total_number: usize = 0;

//                     for single_file in multi_file {
//                         if single_file.number != 0 {
//                             total_number += single_file.number;

//                             println!(
//                                 "{}: {}",
//                                 single_file.path.display().yellow(),
//                                 single_file.number
//                             );
//                         }
//                     }

//                     if total_number == 0 {
//                         let not_fount = "Not Found".red();
//                         return Err(Error::NotFound(not_fount));
//                     }

//                     println!("Total Match Number: {total_number}");
//                 }
//             },
//         },
//     }

//     Ok(())
// }

#[derive(Debug)]
pub enum SearchResult {
    Normal(Vec<NormalResult>),
    Count(Vec<CountResult>),
}

#[derive(Debug)]
pub struct NormalResult {
    pub path: Option<PathBuf>,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
pub struct Match {
    pub line_num: usize,
    pub content: String,
    pub range: MatchRange,
}

#[derive(Debug)]
pub struct MatchRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct CountResult {
    pub path: Option<PathBuf>,
    pub number: usize,
}

// pub fn _render<W: Write>(
pub fn render(
    // pattern: &str,
    result: &SearchResult,
    // writer: &mut W,
    
) -> Result<(), Error> {
    match result {
        SearchResult::Normal(result) => {
            for (no, normal) in result.iter().enumerate() {
                if let Some(path) = normal.path.clone() && no > 1 {
                    println!("{}", path.display().yellow());
                }

                for single_match in normal.matches.iter() {
                    let content = &single_match.content[single_match.range.start..single_match.range.end];
                    println!(
                        "{}:{}",
                        (single_match.line_num + 1).blue(),
                        single_match.content.replace(content, &content.green()),
                    )
                }

                if no < result.iter().len() -1  {
                    println!()
                }
            }
        }

        SearchResult::Count(result) => {
            for count in result {
                if let Some(path) = count.path.clone() {
                    println!("{}: {}", path.display().yellow(), count.number)
                } else {
                    println!("{}", count.number);
                }
            }
        }
    }

    Ok(())
}
