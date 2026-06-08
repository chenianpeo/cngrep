use crate::config::MatchResult;

pub fn output(result: Vec<MatchResult>) -> Result<(), String> {
    if result.is_empty() {
        return Err("not found content".to_string());
    }

    print!("local in: {} \ncontent is:\n", result[0].path);
    for r in &result {
        println!("  {:<8} | {}", r.line_no, r.content)
    }

    Ok(())
}
