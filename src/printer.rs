use std::{
    io::{self, Write},
    process::ExitCode,
};

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

pub fn output(result: &SearchResult, mode: &OutputMode) -> Result<ExitCode, Error> {
    let stdout = io::stdout();
    let mut writer = stdout.lock();

    let exit_code = match result {
        SearchResult::Normal(normals) => render(normals, &mut writer, mode)?,
        SearchResult::Count(counts) => render_count(counts, &mut writer, mode)?,
    };

    Ok(exit_code)
}

pub fn render<W: Write>(
    normals: &[NormalResult],
    writer: &mut W,
    mode: &OutputMode,
) -> Result<ExitCode, Error> {
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

    Ok(ExitCode::from(0))
}

pub fn render_count<W: Write>(
    counts: &[CountResult],
    writer: &mut W,
    mode: &OutputMode,
) -> Result<ExitCode, Error> {
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

    Ok(ExitCode::from(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Range;
    use std::path::PathBuf;

    fn mode() -> OutputMode {
        OutputMode {
            color: false,
            line_num: false,
        }
    }

    fn normal(path: Option<PathBuf>, content: &str, line_num: usize, range: Range) -> NormalResult {
        NormalResult {
            path,
            matches: vec![crate::matcher::Match {
                line_num,
                content: content.to_string(),
                range,
            }],
        }
    }

    fn count(path: Option<PathBuf>, number: usize) -> CountResult {
        CountResult { path, number }
    }

    #[test]
    fn render_single() {
        let normals = vec![normal(None, "test line", 0, Range { start: 0, end: 4 })];

        let mut output = Vec::new();

        let code = render(&normals, &mut output, &mode()).unwrap();

        assert_eq!(code, ExitCode::from(0));
        assert_eq!(String::from_utf8(output).unwrap(), "test line\n");
    }

    #[test]
    fn render_multiple() {
        let normals = vec![
            normal(
                Some(PathBuf::from("one.txt")),
                "test one",
                0,
                Range { start: 0, end: 4 },
            ),
            normal(
                Some(PathBuf::from("two.txt")),
                "test two",
                0,
                Range { start: 0, end: 4 },
            ),
        ];

        let mut output = Vec::new();

        render(
            &normals,
            &mut output,
            &OutputMode {
                color: false,
                line_num: false,
            },
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            "one.txt\ntest one\n\ntwo.txt\ntest two\n"
        );
    }
    #[test]
    fn render_line_num() {
        let normals = vec![normal(None, "test line", 4, Range { start: 0, end: 4 })];

        let mut output = Vec::new();

        let mode = OutputMode {
            color: false,
            line_num: true,
        };

        render(&normals, &mut output, &mode).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "5:test line\n");
    }

    #[test]
    fn render_path_single() {
        let normals = vec![normal(
            Some(PathBuf::from("test.txt")),
            "test line",
            0,
            Range { start: 0, end: 4 },
        )];

        let mut output = Vec::new();

        render(&normals, &mut output, &mode()).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "test line\n");
    }

    #[test]
    fn render_empty() {
        let normals = Vec::new();
        let mut output = Vec::new();

        let code = render(&normals, &mut output, &mode()).unwrap();

        assert_eq!(code, ExitCode::from(0));
        assert!(output.is_empty());
    }

    #[test]
    fn render_count_single() {
        let counts = vec![count(Some(PathBuf::from("test.txt")), 3)];
        let mut output = Vec::new();

        let code = render_count(&counts, &mut output, &mode()).unwrap();

        assert_eq!(code, ExitCode::from(0));
        assert_eq!(String::from_utf8(output).unwrap(), "test.txt:3\n");
    }

    #[test]
    fn render_count_stdin() {
        let counts = vec![count(None, 3)];
        let mut output = Vec::new();

        render_count(&counts, &mut output, &mode()).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "3\n");
    }

    #[test]
    fn render_count_multiple() {
        let counts = vec![
            count(Some(PathBuf::from("one.txt")), 2),
            count(Some(PathBuf::from("two.txt")), 5),
        ];

        let mut output = Vec::new();

        render_count(&counts, &mut output, &mode()).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), "one.txt:2\ntwo.txt:5\n");
    }

    #[test]
    fn render_count_empty() {
        let counts = Vec::new();
        let mut output = Vec::new();

        let code = render_count(&counts, &mut output, &mode()).unwrap();

        assert_eq!(code, ExitCode::from(0));
        assert!(output.is_empty());
    }

    #[test]
    fn render_color() {
        let normals = vec![normal(None, "test line", 0, Range { start: 0, end: 4 })];

        let mut output = Vec::new();

        let mode = OutputMode {
            color: true,
            line_num: false,
        };

        render(&normals, &mut output, &mode).unwrap();

        let output = String::from_utf8(output).unwrap();

        assert!(output.contains("test"));
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn render_line_num_color() {
        let normals = vec![normal(None, "test", 4, Range { start: 0, end: 4 })];

        let mut output = Vec::new();

        render(
            &normals,
            &mut output,
            &OutputMode {
                color: true,
                line_num: true,
            },
        )
        .unwrap();

        let output = String::from_utf8(output).unwrap();

        assert!(output.contains("5"));
        assert!(output.contains(":"));
        assert!(output.contains("test"));
        assert!(output.contains("\x1b["));
    }

    #[test]
    fn render_count_color() {
        let counts = vec![count(Some(PathBuf::from("test.txt")), 3)];
        let mut output = Vec::new();

        let mode = OutputMode {
            color: true,
            line_num: false,
        };

        render_count(&counts, &mut output, &mode).unwrap();

        let output = String::from_utf8(output).unwrap();

        assert!(output.contains("test.txt"));
        assert!(output.contains("3"));
        assert!(output.contains("\x1b["));
    }
}
