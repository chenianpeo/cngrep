use crate::cli::args;
use crate::config::MatchResult;
use std::io::BufRead;

pub fn run() -> Result<(), String> {
    // conduct arguments
    let cli_args = args()?;

    // read file and content
    let read_result = cli_args.read()?;

    // content search
    let mut search_result: Vec<MatchResult> = Vec::new();

    for (line_no, line) in read_result.file.lines().enumerate() {
        let line = line.unwrap();
        let line_no = line_no + 1;

        if line.contains(&cli_args.query.to_string()) {
            search_result.push(MatchResult {
                line_no,
                content: line,
                path: read_result.path.to_string(),
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
