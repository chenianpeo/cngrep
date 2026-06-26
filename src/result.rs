use std::path::PathBuf;

#[derive(Debug)]
pub struct FileMatch {
    pub path: PathBuf,
    pub line_no: usize,
    pub content: String,
}

#[derive(Debug)]
pub struct StdinMatch {
    pub content: String,
}

#[derive(Debug)]
pub struct DirMatch {
    pub content: Vec<FileMatch>,
}

#[derive(Debug)]
pub enum MatchResult {
    File(Vec<FileMatch>),
    Stdin(Vec<StdinMatch>),
    Dir(Vec<DirMatch>),
}
