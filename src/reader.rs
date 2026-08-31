use std::{
    env,
    fs::{File, read_dir},
    io::{IsTerminal, Read},
    path::{Path, PathBuf},
};

use crate::error::Error;

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

            'inner: for entry in dir_path {
                let entry = entry?;
                let single_path = entry.path();

                let path_str = single_path.to_str().ok_or(Error::NotFound)?;
                let path_single_str: Vec<&str> = path_str.split('/').collect();

                for single_str in path_single_str {
                    if single_str.starts_with('.') {
                        continue 'inner;
                    }

                    if single_str.contains("target") {
                        continue 'inner;
                    }
                }

                result.extend(recursive_path(&[single_path])?);
            }
        }
    }

    Ok(result)
}

fn is_binary(path: &Path) -> Result<bool, Error> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf.contains(&0) || std::str::from_utf8(&buf).is_err())
}

#[warn(unused_variables)]
fn _exclude(_path: &PathBuf) -> bool {
    true
}
