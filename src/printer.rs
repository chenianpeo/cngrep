use std::{io, io::Write};

use crate::{
    common::Color,
    error::Error,
    matcher::{CountResult, NormalResult, SearchResult},
};

#[derive(Debug)]
pub struct OutputMode {
    pub color: bool,
    pub line_num: bool,
}

pub fn output(result: &SearchResult, mode: &OutputMode) -> Result<(), Error> {
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
            let mut line: String;
            let line_num: String;

            if mode.color {
                let content = &single.content[single.range.start..single.range.end];

                line = single.content.replace(content, &content.green());
                line_num = (single.line_num + 1).blue().to_string();
            } else {
                line = single.content.to_string();
                line_num = format!("{}", single.line_num + 1);
            }

            if mode.line_num {
                line = format!("{}:{}", line_num, line);
            }

            writeln!(writer, "{}", line)?;
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
    for count in counts {
        let mut path = if let Some(path) = &count.path {
            format!("{}:", path.display())
        } else {
            String::new()
        };

        if mode.color {
            path = path.yellow()
        }

        writeln!(writer, "{}{}", path, count.number)?;
    }

    Ok(())
}
