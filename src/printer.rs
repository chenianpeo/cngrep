// use crate::result::FileMatch;

// pub trait Print {
//     fn print(&self);
// }

// #[derive(Debug)]
// pub struct FileMatchResult {
//     pub content: Vec<FileMatch>,
// }

// #[derive(Debug)]
// pub struct StdinMatchResult {
//     pub content: Vec<String>,
// }

// #[derive(Debug)]
// pub struct DirMatchResult {}

// impl Print for FileMatchResult {
//     fn print(&self) {
//         if self.content.is_empty() {
//             println!("File: Not Found");
//         }
//         for line in &self.content {
//             println!("{}: {}", line.line_no, line.content);
//         }
//     }
// }

// impl Print for StdinMatchResult {
//     fn print(&self) {
//         if self.content.is_empty() {
//             println!("Stdin: Not Found");
//         }
//         for line in &self.content {
//             println!("{}", line);
//         }
//     }
// }

// impl Print for DirMatchResult {
//     fn print(&self) {
//         println!("Dir: Not Found");
//     }
// }

// use crate::result::{DirMatch, FileMatch, StdinMatch};

// pub trait Print {
//     fn new(&self);
// }

// impl Print for FileMatch {
//     fn new(&self) {
//         println!("this is print file");
//     }
// }

// impl Print for StdinMatch {
//     fn new(&self) {
//         println!("this is print stdin");
//     }
// }

// impl Print for DirMatch {
//     fn new(&self) {
//         println!("this is print directory");
//     }
// }
