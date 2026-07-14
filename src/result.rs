use std::path::PathBuf;

#[derive(Debug)]
pub enum MatchResult {
    Stdin(Vec<MatchStdin>),
    File(Vec<MatchFile>),
    Dir(Vec<MatchDir>),
}

#[derive(Debug)]
pub struct MatchStdin {
    pub content: String,
}

#[derive(Debug)]
pub struct MatchFile {
    pub line_no: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct MatchDir {
    pub path: PathBuf,
    pub file: Vec<MatchFile>,
}
