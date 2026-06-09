use crate::{
    config::{CliArgs, MatchResult, ReadResult},
    error::CnError,
};
use std::io::BufRead;

pub fn search(args: CliArgs, read: ReadResult) -> Result<Vec<MatchResult>, CnError> {
    let mut search_result: Vec<MatchResult> = Vec::new();

    for (line_no, line) in read.file.lines().enumerate() {
        let line = line.unwrap();
        let line_no = line_no + 1;

        if line.contains(&args.query.to_string()) {
            search_result.push(MatchResult {
                line_no,
                content: line,
                path: read.path.to_string(),
            })
        }
    }

    Ok(search_result)
}
