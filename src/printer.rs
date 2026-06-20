use crate::app::FileMatch;
use crate::error::Error;

pub fn print_file_normal(result: Vec<FileMatch>) -> Result<(), Error> {
    for line in result {
        println!("{}:{}", line.line_no, line.content);
    }
    Ok(())
}

pub fn print_stdin(result: Vec<String>) -> Result<(), Error> {
    for line in result {
        println!("{}", line);
    }

    Ok(())
}
