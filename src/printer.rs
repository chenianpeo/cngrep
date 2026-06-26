use crate::result::MatchResult;
pub trait Print {
    fn print(&self) -> Result<(), crate::error::Error>;
}

impl Print for MatchResult {
    fn print(&self) -> Result<(), crate::error::Error> {
        match self {
            MatchResult::File(file) => {
                if file.is_empty() {
                    println!("Not Found");
                }
                for line in file {
                    println!("{}: {}", line.line_no, line.content);
                }
            }

            MatchResult::Stdin(stdin) => {
                if stdin.is_empty() {
                    println!("Not Found");
                }
                for line in stdin {
                    println!("{}", line.content);
                }
            }

            MatchResult::Dir(dir) => {
                println!("this is print {:?}", dir);
            }
        }

        Ok(())
    }
}
