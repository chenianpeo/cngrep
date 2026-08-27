use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::cli::OutputOptions;
use crate::error::Error;

#[derive(Debug)]
pub enum OutputPosition {
    Terminal,
    File(PathBuf),
}

// it's too abstract
// should more simple
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
    pub range: MatchRange,
}

#[derive(Debug)]
pub struct MatchRange {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct CountResult {
    pub path: Option<PathBuf>,
    pub number: usize,
}

#[derive(Debug)]
pub struct OutputConfig {
    pub position: Option<PathBuf>,
    pub color: bool,
}

// control output process, parse output options
// parse like color mode
pub fn output_result(result: &SearchResult, mode: &[OutputOptions]) -> Result<(), Error> {
    // bug: need redesigned output position options
    // the for loop is not required
    let mut output_position = OutputPosition::Terminal;
    for options in mode {
        output_position = match options {
            OutputOptions::File(file) => OutputPosition::File(file.clone()),
            _ => OutputPosition::Terminal,
        }
    }

    match output_position {
        OutputPosition::File(file) => {
            let mut writer = File::create(file)?;
            render(result, &mut writer)?;
        }

        OutputPosition::Terminal => {
            let stdout = io::stdout();
            let mut writer = stdout.lock();

            render(result, &mut writer)?;
        }
    }

    Ok(())
}

// Color output should be control by color mode
// output result through writeln and default include color
// render function don't judge output position
//
// can split to two function, render and render_count
pub fn render<W: Write>(result: &SearchResult, writer: &mut W) -> Result<(), Error> {
    match result {
        SearchResult::Normal(result) => {
            for (no, normal) in result.iter().enumerate() {
                if let Some(path) = &normal.path
                    && result.len() != 1
                // iter() can simplify
                {
                    writeln!(writer, "{}", path.display().yellow())?;
                }

                for single_match in normal.matches.iter() {
                    let content =
                        &single_match.content[single_match.range.start..single_match.range.end];
                    writeln!(
                        writer,
                        "{}:{}",
                        (single_match.line_num + 1).blue(),
                        single_match.content.replace(content, &content.green())
                    )?;
                }

                if no < result.iter().len() - 1 {
                    writeln!(writer)?;
                }
            }
        }

        SearchResult::Count(result) => {
            for count in result {
                if let Some(path) = &count.path {
                    writeln!(writer, "{}: {}", path.display().yellow(), count.number)?;
                } else {
                    writeln!(writer, "{}", count.number)?;
                }
            }
        }
    }

    Ok(())
}
