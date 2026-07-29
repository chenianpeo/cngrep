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

            if _mode.contains(&Mode::CountOnly) {
                let mut match_number: usize = 0;
                for line in stdin.lines() {
                    if line?.contains(pattern) {
                        match_number += 1;
                    }
                }

                return Ok(MatchResult::Count(match_number));
            }

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

            if _mode.contains(&Mode::CountOnly) {
                let mut match_number: usize = 0;

                for line in content.lines() {
                    if line?.contains(pattern) {
                        match_number += 1;
                    }
                }

                return Ok(MatchResult::Count(match_number));
            }

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
            let mut dir_match_number: usize = 0;

            if _mode.contains(&Mode::CountOnly) {
                for single_file in multi_file {
                    let mut file_match_number: usize = 0;

                    let open_file = File::open(single_file)?;
                    let content = BufReader::new(open_file);

                    for line in content.lines() {
                        if line?.contains(pattern) {
                            file_match_number += 1;
                        }
                    }

                    dir_match_number += file_match_number;
                }

                return Ok(MatchResult::Count(dir_match_number));
            }

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
