use crate::cli::args;
use crate::config::{Args, InputSource, Mode};
use crate::error::Error;
use std::path::PathBuf;

use crate::reader::read_file;
use crate::reader::read_stdin;

use crate::matcher::match_file_count;
use crate::matcher::match_file_normal;
use crate::matcher::match_stdin_count;
use crate::matcher::match_stdin_normal;

use crate::printer::print_file_normal;
use crate::printer::print_stdin;

pub fn run() -> Result<(), Error> {
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;

    let args = Args::from_cli(cli)?;

    match (args.mode, args.input_source) {
        (Mode::Normal, InputSource::File(path)) => {
            let normal_file = NormalFile {
                query: args.query,
                file: path,
            };

            let file_content = read_file(normal_file.file)?;
            let file_match = match_file_normal(file_content, normal_file.query)?;
            let _ = print_file_normal(file_match);
        }
        (Mode::CountOnly, InputSource::File(path)) => {
            let normal_file = NormalFile {
                query: args.query,
                file: path,
            };

            let file = read_file(normal_file.file)?;
            let count = match_file_count(file, normal_file.query)?;
            println!("{}", count);
        }
        (Mode::Normal, InputSource::Stdin) => {
            let handle = read_stdin()?;
            let match_result = match_stdin_normal(handle, args.query)?;
            let _ = print_stdin(match_result);
        }
        (Mode::CountOnly, InputSource::Stdin) => {
            let handle = read_stdin()?;
            let count = match_stdin_count(handle, args.query)?;
            println!("{}", count);
        }
        _ => {}
    }
    Ok(())
}

pub struct NormalFile {
    query: String,
    file: PathBuf,
}

#[derive(Debug)]
pub struct FileMatch {
    pub line_no: usize,
    pub content: String,
}
