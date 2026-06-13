use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct CliArgs {
    pub query: String,
    pub file: String,
}

#[derive(Debug)]
pub struct ReadResult {
    pub path: String,
    pub file: BufReader<File>,
}
#[derive(Debug)]
pub struct MatchResult {
    pub path: String,
    pub line_no: usize,
    pub content: String,
}
