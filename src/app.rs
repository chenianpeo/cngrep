use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use crate::cli::args;
use crate::config::{Args, InputSource, Mode};
use crate::error::Error;

pub fn run() -> Result<(), Error> {
    let arg: Vec<String> = std::env::args().collect();
    let cli = args(arg)?;

    let args = Args::from_cli(cli)?;

    match (args.mode, args.input_source) {
        (Mode::Normal, InputSource::File(path)) => {
            // read file
            let normal_file = NormalFile {
                query: args.query,
                file: path,
            };

            let file = File::open(normal_file.file)?;
            let file_content = BufReader::new(file);

            // content match
            let mut file_match: Vec<FileMatch> = Vec::new();
            for (line_no, line) in file_content.lines().enumerate() {
                let line = line.unwrap();
                let line_no = line_no + 1;

                if line.contains(&normal_file.query) {
                    file_match.push(FileMatch {
                        line_no,
                        content: line,
                    });
                }
            }

            // print search result
            for line in file_match {
                println!("{}:{}", line.line_no, line.content);
            }
        }
        (Mode::CountOnly, InputSource::File(path)) => {
            let normal_file = NormalFile {
                query: args.query,
                file: path,
            };

            let file = File::open(normal_file.file)?;
            let file_content = BufReader::new(file);

            let mut count: usize = 0;
            for line in file_content.lines() {
                let line = line?;
                if line.contains(&normal_file.query) {
                    count += 1;
                }
            }
            println!("{}", count);
        }
        (Mode::Normal, InputSource::Stdin) => {
            let buf = io::stdin();
            let handle = buf.lock();

            let mut match_result: Vec<String> = Vec::new();

            for line in handle.lines() {
                let line = line?;

                if line.contains(&args.query) {
                    match_result.push(line);
                }
            }

            for line in match_result {
                println!("{}", line);
            }
        }
        (Mode::CountOnly, InputSource::Stdin) => {
            let buf = io::stdin();
            let handle = buf.lock();

            // let mut match_result: Vec<String> = Vec::new();
            let mut count: usize = 0;

            for line in handle.lines() {
                let line = line?;

                if line.contains(&args.query) {
                    count += 1;
                }
            }

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
    line_no: usize,
    content: String,
}
