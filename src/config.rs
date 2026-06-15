use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliArgs {
    pub query: String,
    pub file: String,
    pub mode: Mode,
}

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
