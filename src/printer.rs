use crate::config::MatchResult;

pub fn output(result: Vec<MatchResult>) {
    if result.is_empty() {
        println!("not found content");
    } else {
        print!("local in: {} \ncontent is:\n", result[0].path);
        for r in &result {
            println!("  {:<8} | {}", r.line_no, r.content)
        }
    }
}
