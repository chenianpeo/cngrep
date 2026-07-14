use crate::{error::Error, result::MatchResult};
use owo_colors::OwoColorize;

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
