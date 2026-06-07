#[derive(Debug)]
pub struct CliArgs {
    pub query: String,
    pub path: String,
}

#[derive(Debug)]
pub struct MatchResult {
    pub line_no: usize,
    pub content: String,
}