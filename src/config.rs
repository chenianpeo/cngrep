use std::path::PathBuf;

#[derive(Debug)]
pub struct Cli {
    pub query: String,
    pub file: Option<PathBuf>,

    pub count: bool,
}

#[derive(Debug)]
pub struct Args {
    pub query: String,
    pub input_source: InputSource,
    pub mode: Mode,
}

#[derive(Debug)]
pub enum InputSource {
    File(PathBuf),
    Stdin,
    CurrentDir,
}

#[derive(Debug)]
pub enum Mode {
    Normal,
    CountOnly,
}

#[derive(Debug)]
pub struct MatchResult {
    pub line_no: usize,
    pub content: String,
}
