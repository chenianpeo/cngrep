use std::{
    env,
    io::IsTerminal,
    path::{Path, PathBuf},
};

use crate::{cli::Mode, error::Error};

#[derive(Debug)]
pub enum ReadResult {
    Stdin,
    File(PathBuf),
    MultiFile(Vec<PathBuf>),
}

pub fn read(input_source: &[PathBuf], _mode: &[Mode]) -> Result<ReadResult, Error> {
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            let result = _recursive_dir(&env::current_dir()?, _mode)?;
            Ok(ReadResult::MultiFile(result))
        } else {
            Ok(ReadResult::Stdin)
        }
    } else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            let result = _recursive_dir(&input_source[0], _mode)?;
            Ok(ReadResult::MultiFile(result))
        } else {
            Ok(ReadResult::File(input_source[0].clone()))
        }
    } else {
        let result = input_source
            .iter()
            .map(|entry| {
                let path = entry.clone();
                Ok(path)
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(ReadResult::MultiFile(result))
    }
}

pub fn _recursive_dir(dir: &Path, _mode: &[Mode]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        if let Some(path) = path.to_str()
            && (path.contains(".git") | path.contains("target"))
        {
            continue;
        }

        if path.is_dir() {
            for file in _recursive_dir(&path, _mode)? {
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
