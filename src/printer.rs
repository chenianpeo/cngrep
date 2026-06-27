/*
output layer
include plain render, count render or json render
*/

use crate::{cli::Mode, result::MatchResult};
pub trait Print {
    fn print(&self) -> Result<(), crate::error::Error>;
}

impl Print for NeedPrint {
    fn print(&self) -> Result<(), crate::error::Error> {
        if let Mode::CountOnly = self.mode {
            match &self.result {
                MatchResult::File(file) => {
                    println!("{}", file.len());
                }

                MatchResult::Stdin(stdin) => {
                    println!("{}", stdin.len());
                }

                MatchResult::Dir(dir) => {
                    println!("{}", dir.len());
                }
            }
        } else {
            match &self.result {
                MatchResult::File(file) => {
                    for line in file {
                        println!("{}: {}", line.line_no, line.content);
                    }
                }

                MatchResult::Stdin(stdin) => {
                    for line in stdin {
                        println!("{}", line.content);
                    }
                }

                MatchResult::Dir(dir) => {
                    for line in dir {
                        println!("{:?}", line.content);
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct NeedPrint {
    pub mode: Mode,
    pub result: MatchResult,
}
