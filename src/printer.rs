use crate::config::FileMatch;
use crate::error::Error;

pub fn print_file_normal(result: Vec<FileMatch>) -> Result<(), Error> {
    if result.is_empty() {
        println!("Not Found");
    }
    for line in result {
        println!("{}:{}", line.line_no, line.content);
    }
    Ok(())
}

pub fn print_stdin(result: Vec<String>) -> Result<(), Error> {
    if result.is_empty() {
        println!("Not Found");
    }
    for line in result {
        println!("{}", line);
    }
    Ok(())
}
