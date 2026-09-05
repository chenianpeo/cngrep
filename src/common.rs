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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_red() {
        assert_eq!("test".red(), "\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn color_green() {
        assert_eq!("test".green(), "\x1b[32mtest\x1b[0m");
    }

    #[test]
    fn color_yellow() {
        assert_eq!("test".yellow(), "\x1b[33mtest\x1b[0m");
    }

    #[test]
    fn color_blue() {
        assert_eq!("test".blue(), "\x1b[34mtest\x1b[0m");
    }

    #[test]
    fn color_custom() {
        assert_eq!("test".color(35), "\x1b[35mtest\x1b[0m");
    }

    #[test]
    fn range_values() {
        let range = Range { start: 1, end: 5 };

        assert_eq!(range.start, 1);
        assert_eq!(range.end, 5);
    }
}
