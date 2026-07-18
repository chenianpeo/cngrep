use std::fmt::Display;

use crate::{error::Error, result::MatchResult};

pub fn render(result: &MatchResult) -> Result<(), Error> {
    match result {
        MatchResult::Stdin(stdin_result) => {
            if stdin_result.is_empty() {
                return {
                    println!("{}", "Not Found".red());
                    Ok(())
                };
            }

            for stdin in stdin_result {
                println!("{}", stdin.content);
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
                println!("{}:{}", (file.line_no + 1).blue(), file.content);
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
                    println!("{}:{}", (file.line_no + 1).blue(), file.content);
                }

                if dir_no != dir_result.len() - 1 {
                    println!();
                }
            }
        }
    }

    Ok(())
}

// pub trait Color {
//     fn red(&self) -> String;
//     fn blue(&self) -> String;
//     fn yellow(&self) -> String;
// }

// impl<T: Display> Color for T {
//     fn red(&self) -> String {
//         format!("\x1b[31m{}\x1b[0m", self)
//     }

//     fn blue(&self) -> String {
//         format!("\x1b[34m{}\x1b[0m", self)
//     }

//     fn yellow(&self) -> String {
//         format!("\x1b[33m{}\x1b[0m", self)
//     }
// }

pub trait Color: Display {
    fn color(&self, code: u8) -> String {
        format!("\x1b[{}m{}\x1b[0m", code, self)
    }

    fn red(&self) -> String {
        self.color(31)
    }

    fn yellow(&self) -> String {
        self.color(33)
    }

    fn blue(&self) -> String {
        self.color(34)
    }
}

impl<T: Display> Color for T {}
