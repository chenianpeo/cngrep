use std::{
    env::{self, current_dir},
    fs::{File, read_dir},
    io::{self, IsTerminal, Read},
    path::{Path, PathBuf},
};

use crate::{error::Error, reader::Input::MultiFile};

#[derive(Debug, PartialEq)]
pub enum ReadResult {
    Stdin,
    File(PathBuf),
    MultiFile(Vec<PathBuf>),
}

pub fn read(input_source: &[PathBuf]) -> Result<ReadResult, Error> {
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            Ok(ReadResult::MultiFile(recursive_path(&[
                env::current_dir()?
            ])?))
        } else {
            Ok(ReadResult::Stdin)
        }
    } else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            Ok(ReadResult::MultiFile(recursive_path(input_source)?))
        } else if is_binary(&input_source[0])? {
            Err(Error::UnFinished)
        } else {
            Ok(ReadResult::File(input_source[0].clone()))
        }
    } else {
        Ok(ReadResult::MultiFile(recursive_path(input_source)?))
    }
}

fn recursive_path(paths: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    for path in paths {
        if path.is_file() {
            if is_binary(path)? {
                continue;
            }

            result.push(path.clone());

            continue;
        }

        if path.is_dir() {
            let dir_path = read_dir(path)?;

            for entry in dir_path {
                let entry = entry?;
                let single_path = entry.path();

                if is_exclude(&single_path)? {
                    continue;
                }

                result.extend(recursive_path(&[single_path])?);
            }
        }
    }

    Ok(result)
}

/// analyze input path file types
/// performance loss is quite serious due to read total files
fn is_binary(path: &Path) -> Result<bool, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf.contains(&0) || std::str::from_utf8(&buf).is_err())
}

/// exclude file and directory
fn is_exclude(path: &Path) -> Result<bool, Error> {
    let components = path.components();

    use std::ffi::OsStr;

    for component in components.as_path() {
        if component.to_string_lossy().starts_with(".") {
            return Ok(true);
        }

        if component == OsStr::new("target") {
            return Ok(true);
        }
    }

    Ok(false)
}

#[derive(Debug)]
pub enum Input {
    Stdin,
    MultiFile(Vec<PathBuf>),
}

pub fn new_read(path: &[PathBuf]) -> Result<Input, Error> {
    let stdin = io::stdin();
    if !stdin.is_terminal() {
        return Ok(Input::Stdin);
    }

    if stdin.is_terminal() && path.is_empty() {
        return Ok(Input::MultiFile(recursive_path(&[current_dir()?])?));
    }

    if let Some(file) = path.first()
        && file.is_file()
        && path.len() == 1
    {
        return Ok(MultiFile(vec![file.to_path_buf()]));
    }

    if let Some(dir) = path.first()
        && dir.is_dir()
        && path.len() == 1
    {
        return Ok(MultiFile(recursive_path(std::slice::from_ref(dir))?));
    }

    Ok(MultiFile(recursive_path(path)?))
}
