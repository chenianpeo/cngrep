use crate::cli::args;
use crate::config::MatchResult;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

pub fn run() -> Result<(), String> {
    // conduct arguments
    let cli_args = args()?;

    // read file and content
    let open_file = File::open(&cli_args.path).unwrap();
    let file_content = BufReader::new(open_file);

    let absolute_path = fs::canonicalize(cli_args.path).unwrap();
    let file_path = absolute_path.to_str().unwrap();

    // content search
    let mut search_result: Vec<MatchResult> = Vec::new();

    for (line_no, line) in file_content.lines().enumerate() {
        let line = line.unwrap();
        let line_no = line_no + 1;

        if line.contains(&cli_args.query.to_string()) {
            search_result.push(MatchResult {
                line_no,
                content: line,
                path: file_path.to_string(),
            });
        }
    }

    // result output
    if search_result.is_empty() {
        return Err("not find content".to_string());
    }

    print!("local in: {} \ncontent is:\n", search_result[0].path);
    for result in &search_result {
        println!("   {:<8} | {}", result.line_no, result.content)
    }

    Ok(())
}
