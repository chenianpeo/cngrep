use crate::{config::MatchResult, error::CnError};

pub fn output(result: Vec<MatchResult>) -> Result<(), CnError> {
    if result.is_empty() {
        return Err(CnError::Custom("not found content".to_string()));
    }

    print!("local in: {} \ncontent is:\n", result[0].path);
    for r in &result {
        println!("  {:<8} | {}", r.line_no, r.content)
    }

    Ok(())
}
