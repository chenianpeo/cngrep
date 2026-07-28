use std::{
    env,
    fs::File,
    io::{BufReader, IsTerminal, Stdin},
    path::{Path, PathBuf},
};

use crate::{cli::Mode, error::Error};

#[derive(Debug)]
pub enum ReadResult {
    Stdin(BufReader<Stdin>),
    MultiFile(Vec<ReadFile>),
    File(ReadFile),
}

#[derive(Debug)]
pub struct ReadFile {
    pub path: PathBuf,
    pub reader: BufReader<File>,
}

pub fn read(input_source: &[PathBuf], _mode: &[Mode]) -> Result<ReadResult, Error> {
    // need judge stdin or current path when vec is empty
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            let result = recursive_directory(&env::current_dir()?, _mode)?;

            Ok(ReadResult::MultiFile(result))
        } else {
            Ok(ReadResult::Stdin(BufReader::new(stdin)))
        }
    } else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            let result = recursive_directory(&input_source[0], _mode)?;

            Ok(ReadResult::MultiFile(result))
        } else {
            let file = File::open(input_source[0].clone())?;
            Ok(ReadResult::File(ReadFile {
                path: input_source[0].clone(),
                reader: BufReader::new(file),
            }))
        }
    } else {
        let result = input_source
            .iter()
            .map(|entry| {
                let path = entry;
                let file = File::open(entry)?;
                Ok(ReadFile {
                    path: path.to_path_buf(),
                    reader: BufReader::new(file),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(ReadResult::MultiFile(result))
    }
}

// recursion directory
fn recursive_directory(dir: &Path, _mode: &[Mode]) -> Result<Vec<ReadFile>, Error> {
    let mut result: Vec<ReadFile> = Vec::new();

    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();

        // skip .git or other directory
        if let Some(path) = path.to_str()
            && path.contains(".git")
        {
            continue;
        }

        // recursion open dir
        if path.is_dir() {
            for file in recursive_directory(&path, _mode)? {
                result.push(file);
            }
            continue;
        }

        if !path.is_file() {
            continue;
        }

        // skip nonsupport file type
        if matches!(
            path.extension().and_then(|s| s.to_str()),
            Some("pdf" | "epub")
        ) {
            continue;
        }

        let file = File::open(&path)?;
        result.push(ReadFile {
            path,
            reader: BufReader::new(file),
        });
    }

    Ok(result)
}
