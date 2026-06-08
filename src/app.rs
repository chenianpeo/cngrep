use crate::cli::args;
use crate::matcher::search;

pub fn run() -> Result<(), String> {
    // conduct arguments
    let cli_args = args()?;

    // read file and content
    let read_result = cli_args.read()?;

    // content search
    let search_result = search(cli_args, read_result)?;

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
