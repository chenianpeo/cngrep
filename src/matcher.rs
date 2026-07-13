use std::io::BufRead;

use crate::{cli::Mode, error::Error, reader::ReadResult};

pub fn search(pattern: &str, read_result: &mut ReadResult, _mode: &Mode) -> Result<(), Error> {
    match read_result {
        ReadResult::Stdin(stdin) => {
            for line in stdin.lines() {
                let line = line?;
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }

        ReadResult::File(file) => {
            println!("{}", file.path.canonicalize()?.display());

            for (line_no, line) in (&mut file.reader).lines().enumerate() {
                let line = line?;
                if line.contains(pattern) {
                    println!("{}: {}", line_no + 1, line);
                }
            }
        }

        ReadResult::Dir(dir) => {
            for file in dir {
                // judge support file type
                if matches!(
                    file.path.extension().and_then(|s| s.to_str()),
                    Some("pdf" | "epub")
                ) {
                    continue;
                }

                // println file and result
                println!("{}", file.path.canonicalize()?.display());

                // match content
                for (line_no, line) in (&mut file.reader).lines().enumerate() {
                    let line = line?;
                    if line.contains(pattern) {
                        println!("{}: {}", line_no + 1, line);
                    }
                }

                println!();
            }
        }
    }

    Ok(())
}
