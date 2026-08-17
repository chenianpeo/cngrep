use std::{env, io::IsTerminal, path::PathBuf};

use crate::{cli::ReadOptions, error::Error};

#[derive(Debug, PartialEq)]
pub enum ReadResult {
    Stdin,
    File(PathBuf),
    MultiFile(Vec<PathBuf>),
}

pub fn read(input_source: &[PathBuf], _mode: &[ReadOptions]) -> Result<ReadResult, Error> {
    // stdin or current directory
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            Ok(ReadResult::MultiFile(recursive_path(&[
                env::current_dir()?
            ])?))
        } else {
            Ok(ReadResult::Stdin)
        }
    }
    // single file or directory
    else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            Ok(ReadResult::MultiFile(recursive_path(input_source)?))
        } else {
            Ok(ReadResult::File(input_source[0].clone()))
        }
    }
    // multiple file or directory
    else {
        Ok(ReadResult::MultiFile(recursive_path(input_source)?))
    }
}

fn recursive_path(path: &[PathBuf]) -> Result<Vec<PathBuf>, Error> {
    let mut result: Vec<PathBuf> = Vec::new();

    'outer: for path in path {
        let path_str = path.to_str().ok_or(Error::NotFound("NotFound".into()))?;
        let path_single_str: Vec<&str> = path_str.split('/').collect();

        // ignore file or directory
        for single_str in path_single_str {
            if single_str.starts_with('.') {
                continue 'outer;
            }

            if single_str.contains("target") {
                continue 'outer;
            }

            if single_str.ends_with(".pdf") || single_str.ends_with(".epub") {
                continue 'outer;
            }
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

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_dir() -> PathBuf {
        let mut path = std::env::temp_dir();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        path.push(format!("cngrep_test_{}", timestamp));

        fs::create_dir(&path).unwrap();

        path
    }

    #[test]
    fn read_one_file() {
        let dir = create_temp_dir();
        let file = dir.join("content.txt");

        fs::write(&file, "hello cngrep").unwrap();

        let actual = read(&[file.clone()], &[]).unwrap();

        let expected = ReadResult::File(file);

        assert_eq!(actual, expected);

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn read_one_dir() {
        let dir = create_temp_dir();

        let sub_dir = dir.join("test");

        fs::create_dir(&sub_dir).unwrap();

        let file1 = sub_dir.join("test1.txt");
        let file2 = sub_dir.join("test2.txt");

        fs::write(&file1, "contents").unwrap();
        fs::write(&file2, "contents").unwrap();

        let actual = read(&[dir.clone()], &[]).unwrap();

        match actual {
            ReadResult::MultiFile(mut files) => {
                files.sort();

                let mut expected = vec![file1, file2];

                expected.sort();

                assert_eq!(files, expected);
            }
            _ => panic!("expected multi file"),
        }

        fs::remove_dir_all(dir).unwrap();
    }
}
