use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::{
    cli::Mode,
    error::Error,
    reader::ReadResult,
    result::{MatchDir, MatchFile, MatchResult, MatchStdin},
};

pub fn search(
    pattern: &str,
    read_result: &ReadResult,
    _mode: &[Mode],
) -> Result<MatchResult, Error> {
    let result: MatchResult = match read_result {
        ReadResult::Stdin => {
            let stdin = std::io::stdin();
            let mut match_result: Vec<MatchStdin> = Vec::new();

            for line in stdin.lines() {
                let line = line?;
                if line.contains(pattern) {
                    match_result.push(MatchStdin { content: line });
                }
            }

            MatchResult::Stdin(match_result)
        }

        ReadResult::File(file) => {
            let mut match_result: Vec<MatchFile> = Vec::new();

            let file = File::open(file)?;
            let content = BufReader::new(file);

            for (line_no, line) in content.lines().enumerate() {
                let line = line?;

                if line.contains(pattern) {
                    match_result.push(MatchFile {
                        line_no,
                        content: line,
                    });
                }
            }

            MatchResult::File(match_result)
        }

        ReadResult::MultiFile(multi_file) => {
            let mut dir_match_result: Vec<MatchDir> = Vec::new();

            for single_file in multi_file {
                let mut file_match_result: Vec<MatchFile> = Vec::new();

                let open_file = File::open(single_file)?;
                let content = BufReader::new(open_file);

                for (line_no, line) in content.lines().enumerate() {
                    let line = line?;

                    if line.contains(pattern) {
                        file_match_result.push(MatchFile {
                            line_no,
                            content: line,
                        });
                    }
                }

                if !file_match_result.is_empty() {
                    dir_match_result.push(MatchDir {
                        path: single_file.canonicalize()?,
                        file: file_match_result,
                    });
                }
            }

            MatchResult::Dir(dir_match_result)
        }
    };

    Ok(result)
}
