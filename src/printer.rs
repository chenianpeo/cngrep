use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::common::{Color, Range};
use crate::error::Error;

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

#[derive(Debug)]
pub struct CountResult {
    pub path: Option<PathBuf>,
    pub number: usize,
}

#[derive(Debug)]
pub struct OutputMode {
    pub color: bool,
    pub line_num: bool,
}

pub fn output_result(result: &SearchResult, mode: &OutputMode) -> Result<(), Error> {
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    match result {
        SearchResult::Normal(normals) => render(normals, &mut writer, mode)?,
        SearchResult::Count(counts) => render_count(counts, &mut writer, mode)?,
    }

    Ok(())
}

pub fn render<W: Write>(
    normals: &[NormalResult],
    writer: &mut W,
    mode: &OutputMode,
) -> Result<(), Error> {
    for (no, normal) in normals.iter().enumerate() {
        if let Some(path) = &normal.path
            && normals.len() != 1
        {
            if mode.color {
                writeln!(writer, "{}", path.display().yellow())?;
            } else {
                writeln!(writer, "{}", path.display())?;
            }
        }

        for single in &normal.matches {
            if mode.color {
                let content = &single.content[single.range.start..single.range.end];
                writeln!(
                    writer,
                    "{}:{}",
                    (single.line_num + 1).blue(),
                    single.content.replace(content, &content.green())
                )?
            } else {
                writeln!(writer, "{}:{}", (single.line_num + 1), single.content)?;
            }
        }

        if no < normals.len() - 1 {
            writeln!(writer)?;
        }
    }

    Ok(())
}

pub fn render_count<W: Write>(
    counts: &[CountResult],
    writer: &mut W,
    mode: &OutputMode,
) -> Result<(), Error> {
    if mode.color {
        for count in counts {
            if let Some(path) = &count.path {
                writeln!(writer, "{}: {}", path.display().yellow(), count.number)?;
            } else {
                writeln!(writer, "{}", count.number)?;
            }
        }
    } else {
        for count in counts {
            if let Some(path) = &count.path {
                writeln!(writer, "{}: {}", path.display(), count.number)?;
            } else {
                writeln!(writer, "{}", count.number)?;
            }
        }
    }

    Ok(())
}
