use std::io::BufRead;

use crate::{
    cli::Mode,
    error::Error,
    reader::ReadResult,
    result::{MatchDir, MatchFile, MatchResult, MatchStdin},
};

pub fn search(
    pattern: &str,
    read_result: &mut ReadResult,
    _mode: &Mode,
) -> Result<MatchResult, Error> {
    let result: MatchResult = match read_result {
        ReadResult::Stdin(stdin) => {
            let mut result: Vec<MatchStdin> = Vec::new();

            for line in stdin.lines() {
                let line = line?;
                if line.contains(pattern) {
                    result.push(MatchStdin { content: line });
                }
            }

            MatchResult::Stdin(result)
        }

        ReadResult::File(file) => {
            let mut result: Vec<MatchFile> = Vec::new();

            for (line_no, line) in (&mut file.reader).lines().enumerate() {
                let line = line?;
                if line.contains(pattern) {
                    result.push(MatchFile {
                        line_no,
                        content: line,
                    });
                }
            }

            MatchResult::File(result)
        }

        ReadResult::Dir(dir) => {
            let mut result: Vec<MatchDir> = Vec::new();

            for file in dir {
                let mut file_result: Vec<MatchFile> = Vec::new();

                for (line_no, line) in (&mut file.reader).lines().enumerate() {
                    let line = line?;

                    if line.contains(pattern) {
                        file_result.push(MatchFile {
                            line_no,
                            content: line,
                        });
                    }
                }

                if !file_result.is_empty() {
                    result.push(MatchDir {
                        path: file.path.canonicalize()?,
                        file: file_result,
                    });
                }
            }

            MatchResult::Dir(result)
        }
    };

    Ok(result)
}
