use crate::config::MatchResult;

pub fn output(result: Vec<MatchResult>) {
    if result.is_empty() {
        println!("not found content");
    } else {
        let mut return_string = format!("local in: {} \ncontent is:\n", result[0].path);
        for r in &result {
            return_string = format!("{}\n  {:<8} | {}", return_string, r.line_no, r.content);
        }

        println!("{return_string}");
    }
}
