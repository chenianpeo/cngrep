use std::io::Write;
use std::path::PathBuf;
use std::{fmt::Display, fs::File};

// TODO search result shouldn't include match type
#[derive(Debug)]
pub enum SearchResult {
    Normal(NormalResult), // default match
    Count(CountResult),   // match of count only
    IgnoreCase(IgnoreCaseResult),
}

#[derive(Debug)]
pub enum NormalResult {
    StdinFile(Vec<MatchStdinFile>), // stdin and single file
    MultiFile(Vec<MatchMultiFile>), // multiple file and dir
}

#[derive(Debug)]
pub enum CountResult {
    StdinFile(usize),
    MultiFile(Vec<CountMultiFile>),
}

#[derive(Debug)]
pub enum IgnoreCaseResult {
    StdinFile(Vec<MatchStdinFile>),
    MultiFile(Vec<MatchMultiFile>),
}

#[derive(Debug)]
pub struct MatchStdinFile {
    pub line_no: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct MatchMultiFile {
    pub path: PathBuf,
    pub file: Vec<MatchStdinFile>,
}

#[derive(Debug)]
pub struct CountMultiFile {
    pub path: PathBuf,
    pub number: usize,
}

use crate::cli::OutputOptions;
use crate::error::Error;

fn is_matched<T>(r: &[T]) -> Result<(), Error> {
    let not_fount = "Not Found".red();

    if r.is_empty() {
        return Err(Error::NotFound(not_fount));
    }

    Ok(())
}

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

// pub trait Print {
//     fn new(read_result: &ReadResult) -> Self;
// }

// TODO output type of terminal and file have refactor
// output result
pub fn render(pattern: &str, result: &SearchResult, mode: &[OutputOptions]) -> Result<(), Error> {
    // output result to file
    let output_file = mode.iter().find_map(|mode| match mode {
        OutputOptions::OutputFile(path) => Some(path),
        _ => None,
    });

    if let Some(path) = output_file {
        let mut output_file = File::create(path)?;

        match result {
            SearchResult::Normal(normal_result) => match normal_result {
                NormalResult::StdinFile(file_result) => {
                    is_matched(file_result)?;

                    for file in file_result {
                        writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
                    }
                }

                NormalResult::MultiFile(dir_result) => {
                    is_matched(dir_result)?;

                    for (dir_no, dir) in dir_result.iter().enumerate() {
                        writeln!(output_file, "{}", dir.path.display())?;

                        for file in dir.file.iter() {
                            writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
                        }

                        if dir_no != dir_result.len() - 1 {
                            writeln!(output_file)?;
                        }
                    }
                }
            },

            SearchResult::Count(count_result) => match count_result {
                CountResult::StdinFile(stdin_file) => {
                    if stdin_file == &0 {
                        let not_fount = "Not Found".red();
                        return Err(Error::NotFound(not_fount));
                    } else {
                        writeln!(output_file, "{stdin_file}")?;
                    }
                }

                CountResult::MultiFile(multi_file) => {
                    let mut total_number: usize = 0;

                    for single_file in multi_file {
                        if single_file.number != 0 {
                            total_number += single_file.number;
                            writeln!(
                                output_file,
                                "{}:{}",
                                single_file.path.display(),
                                single_file.number
                            )?;
                        }
                    }

                    if total_number == 0 {
                        let not_fount = "Not Found".red();
                        return Err(Error::NotFound(not_fount));
                    }

                    writeln!(output_file, "Total Match Number: {total_number}")?;
                }
            },

            SearchResult::IgnoreCase(ignore_case_result) => match ignore_case_result {
                IgnoreCaseResult::StdinFile(file_result) => {
                    is_matched(file_result)?;

                    for file in file_result {
                        writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
                    }
                }

                IgnoreCaseResult::MultiFile(dir_result) => {
                    is_matched(dir_result)?;

                    for (dir_no, dir) in dir_result.iter().enumerate() {
                        writeln!(output_file, "{}", dir.path.display())?;

                        for file in dir.file.iter() {
                            writeln!(output_file, "{}:{}", file.line_no + 1, file.content,)?;
                        }

                        if dir_no != dir_result.len() - 1 {
                            writeln!(output_file)?;
                        }
                    }
                }
            },
        }
    } else {
        // output result to terminal
        match result {
            SearchResult::Normal(normal_result) => match normal_result {
                NormalResult::StdinFile(file_result) => {
                    is_matched(file_result)?;

                    for file in file_result {
                        println!(
                            "{}:{}",
                            (file.line_no + 1).blue(),
                            file.content.replace(pattern, &pattern.green())
                        );
                    }
                }

                NormalResult::MultiFile(dir_result) => {
                    is_matched(dir_result)?;

                    for (dir_no, dir) in dir_result.iter().enumerate() {
                        println!("{}", dir.path.display().yellow());

                        for file in dir.file.iter() {
                            println!(
                                "{}:{}",
                                (file.line_no + 1).blue(),
                                file.content.replace(pattern, &pattern.green())
                            );
                        }

                        if dir_no != dir_result.len() - 1 {
                            println!();
                        }
                    }
                }
            },

            SearchResult::Count(count_result) => match count_result {
                CountResult::StdinFile(stdin_file) => {
                    if stdin_file == &0 {
                        let not_fount = "Not Found".red();
                        return Err(Error::NotFound(not_fount));
                    }
                    println!("{stdin_file}")
                }

                CountResult::MultiFile(multi_file) => {
                    let mut total_number: usize = 0;

                    for single_file in multi_file {
                        if single_file.number != 0 {
                            total_number += single_file.number;

                            println!(
                                "{}: {}",
                                single_file.path.display().yellow(),
                                single_file.number
                            );
                        }
                    }

                    if total_number == 0 {
                        let not_fount = "Not Found".red();
                        return Err(Error::NotFound(not_fount));
                    }

                    println!("Total Match Number: {total_number}");
                }
            },

            SearchResult::IgnoreCase(ignore_case_result) => match ignore_case_result {
                IgnoreCaseResult::StdinFile(file_result) => {
                    is_matched(file_result)?;
                    let pattern_lowercase = pattern.to_lowercase();
                    let pattern_len = pattern_lowercase.len();

                    for file in file_result {
                        let file_content = file.content.clone();
                        let file_content_lowercase = file_content.to_lowercase();
                        let mut index_pattern = 0;
                        if let Some(index) = file_content_lowercase.find(&pattern_lowercase) {
                            index_pattern = index;
                        }
                        let index_pattern_end = index_pattern + pattern_len;
                        let match_content = &file_content[index_pattern..index_pattern_end];

                        println!(
                            "{}:{}",
                            (file.line_no + 1).blue(),
                            file_content.replace(match_content, &match_content.green())
                        );
                    }
                }

                IgnoreCaseResult::MultiFile(dir_result) => {
                    is_matched(dir_result)?;
                    let pattern_lowercase = pattern.to_lowercase();
                    let pattern_len = pattern_lowercase.len();

                    for (dir_no, dir) in dir_result.iter().enumerate() {
                        println!("{}", dir.path.display().yellow());

                        for file in dir.file.iter() {
                            let file_content = file.content.clone();
                            let file_content_lowercase = file_content.to_lowercase();
                            let mut index_pattern = 0;
                            if let Some(index) = file_content_lowercase.find(&pattern_lowercase) {
                                index_pattern = index;
                            }
                            let index_pattern_end = index_pattern + pattern_len;
                            let match_content = &file_content[index_pattern..index_pattern_end];

                            println!(
                                "{}:{}",
                                (file.line_no + 1).blue(),
                                file_content.replace(match_content, &match_content.green())
                            );
                        }

                        if dir_no != dir_result.len() - 1 {
                            println!();
                        }
                    }
                }
            },
        }
    }

    Ok(())
}
