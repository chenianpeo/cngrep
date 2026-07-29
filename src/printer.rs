use std::fmt::Display;

use crate::{
    cli::Mode,
    error::Error,
    result::MatchResult::{self, Count},
};

pub fn render(_pattern: &str, result: &MatchResult, _mode: &[Mode]) -> Result<(), Error> {
    match result {
        MatchResult::Stdin(stdin_result) => {
            if stdin_result.is_empty() {
                return {
                    println!("{}", "Not Found".red());
                    Ok(())
                };
            }

            for stdin in stdin_result {
                println!("{}", stdin.content.replace(_pattern, &_pattern.green()));
            }
        }
        MatchResult::File(file_result) => {
            if file_result.is_empty() {
                return {
                    println!("{}", "Not Found".red());
                    Ok(())
                };
            }

            for file in file_result {
                println!(
                    "{}:{}",
                    (file.line_no + 1).blue(),
                    file.content.replace(_pattern, &_pattern.green())
                );
            }
        }
        MatchResult::Dir(dir_result) => {
            if dir_result.is_empty() {
                return {
                    println!("{}", "Not Found".red());
                    Ok(())
                };
            }

            for (dir_no, dir) in dir_result.iter().enumerate() {
                println!("{}", dir.path.display().yellow());

                for file in dir.file.iter() {
                    println!(
                        "{}:{}",
                        (file.line_no + 1).blue(),
                        file.content.replace(_pattern, &_pattern.green())
                    );
                }

                if dir_no != dir_result.len() - 1 {
                    println!();
                }
            }
        }

        Count(match_number) => {
            println!("{match_number}");
        }
    }

    Ok(())
}

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
