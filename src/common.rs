/*
defined common interface
*/

use std::fmt::Display;

pub trait Color: Display {
    fn color(&self, code: u8) -> String {
        format!("\x1b[{}m{}\x1b[0m", code, self)
    }

    fn red(&self) -> String {
        self.color(31)
    }
    fn green(&self) -> String {
        self.color(32)
    }
    fn yellow(&self) -> String {
        self.color(33)
    }
    fn blue(&self) -> String {
        self.color(34)
    }
}

impl<T: Display> Color for T {}

#[derive(Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
