use crate::config::FileMatch;

pub trait Print {
    fn print(&self);
}

#[derive(Debug)]
pub struct FileMatchResult {
    pub content: Vec<FileMatch>,
}

#[derive(Debug)]
pub struct StdinMatchResult {
    pub content: Vec<String>,
}

#[derive(Debug)]
pub struct DirMatchResult {}

// pub struct FileMatchResult {}
impl Print for FileMatchResult {
    fn print(&self) {
        for line in &self.content {
            println!("{}: {}", line.line_no, line.content);
        }
    }
}

impl Print for StdinMatchResult {
    fn print(&self) {
        for line in &self.content {
            println!("{}", line);
        }
    }
}

impl Print for DirMatchResult {
    fn print(&self) {
        println!("{:?}", self);
    }
}
