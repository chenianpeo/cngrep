use std::{
    env,
    io::IsTerminal,
    path::{Path, PathBuf},
};

use crate::{cli::Mode, error::Error};

#[derive(Debug, PartialEq)]
pub enum ReadResult {
    Stdin,
    File(PathBuf),
    MultiFile(Vec<PathBuf>),
}

pub fn read(input_source: &[PathBuf], _mode: &[Mode]) -> Result<ReadResult, Error> {
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            let result = recursive_dir(&env::current_dir()?, _mode)?;
            Ok(ReadResult::MultiFile(result))
        } else {
            Ok(ReadResult::Stdin)
        }
    } else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            let result = recursive_dir(&input_source[0], _mode)?;
            Ok(ReadResult::MultiFile(result))
        } else {
            Ok(ReadResult::File(input_source[0].clone()))
        }
    } else {
        let result = recursive_path(input_source)?;

        Ok(ReadResult::MultiFile(result))
    }
}

fn recursive_path(path: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    for path in path {
        if let Some(path) = path.to_str()
            && (path.contains(".git") | path.contains("/target"))
        {
            continue;
        }

        if matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("pdf" | "epub")
        ) {
            continue;
        }

        if path.is_file() {
            result.push(path.clone());
        }

        if path.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;
                let entry_path = entry.path();

                if matches!(
                    entry_path.extension().and_then(|s| s.to_str()),
                    Some("pdf" | "epub")
                ) {
                    continue;
                }

                if entry_path.is_file() {
                    result.push(entry_path.clone());
                }

                if entry_path.is_dir() {
                    let sub_path = recursive_path(&[entry_path])?;
                    result.extend(sub_path);
                }
            }
        }
    }

    Ok(result)
}

pub fn recursive_dir(dir: &Path, _mode: &[Mode]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if let Some(path) = path.to_str()
            && (path.contains(".git") | path.contains("/target"))
        {
            continue;
        }

        if path.is_dir() {
            for file in recursive_dir(&path, _mode)? {
                result.push(file);
            }
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("pdf" | "epub")
        ) {
            continue;
        }

        result.push(path);
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn path_one_file() {
        let actual = read(
            &[PathBuf::from("/home/cn/Code/cngrep/content.txt")],
            &Vec::new(),
        )
        .unwrap();

        let expected = ReadResult::File(PathBuf::from("/home/cn/Code/cngrep/content.txt"));

        assert_eq!(actual, expected);
    }

    #[test]
    fn path_one_dir() {
        let actual = read(&[PathBuf::from("/home/cn/Documents")], &Vec::new()).unwrap();

        let expected = ReadResult::MultiFile(vec![
            PathBuf::from("/home/cn/Documents/test/main.rs"),
            PathBuf::from("/home/cn/Documents/test.rs"),
        ]);

        assert_eq!(actual, expected);
    }
}
