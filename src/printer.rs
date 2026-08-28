use std::fmt::Display;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::error::Error;

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
pub enum SearchResult {
    Normal(Vec<NormalResult>),
    Count(Vec<CountResult>),
}

#[derive(Debug)]
pub struct NormalResult {
    pub path: Option<PathBuf>,
    pub matches: Vec<Match>,
}

#[derive(Debug)]
pub struct Match {
    pub line_num: usize,
    pub content: String,
    pub range: Range,
}

// public range struct
#[derive(Debug)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct CountResult {
    pub path: Option<PathBuf>,
    pub number: usize,
}

pub fn output_result(result: &SearchResult) -> Result<(), Error> {
    match result {
        SearchResult::Normal(normals) => {
            let stdout = io::stdout();
            let mut writer = stdout.lock();

            render(normals, &mut writer)?;
        }

        SearchResult::Count(counts) => {
            let stdout = io::stdout();
            let mut writer = stdout.lock();

            render_count(counts, &mut writer)?;
        }
    }

    Ok(())
}

pub fn render<W: Write>(normals: &[NormalResult], writer: &mut W) -> Result<(), Error> {
    for (no, normal) in normals.iter().enumerate() {
        if let Some(path) = &normal.path
            && normals.len() != 1
        {
            writeln!(writer, "{}", path.display())?;
        }

        for single in normal.matches.iter() {
            // let content = &single.content[single.range.start..single.range.end];
            writeln!(writer, "{}:{}", (single.line_num + 1), single.content)?;
        }

        if no < normals.len() - 1 {
            writeln!(writer)?;
        }
    }

    Ok(())
}

pub fn render_count<W: Write>(counts: &[CountResult], writer: &mut W) -> Result<(), Error> {
    for count in counts {
        if let Some(path) = &count.path {
            writeln!(writer, "{}: {}", path.display(), count.number)?;
        } else {
            writeln!(writer, "{}", count.number)?;
        }
    }

    Ok(())
}
