use std::{
    env, fs::File, io::{BufReader, IsTerminal, Stdin}, path::PathBuf,
};

use crate::{cli::Mode, error::Error};

#[derive(Debug)]
pub enum ReadResult {
    Stdin(BufReader<Stdin>),
    Dir(Vec<ReadF>),
    File(ReadF),
}

#[derive(Debug)]
pub struct ReadF {
    pub path: PathBuf,
    pub reader: BufReader<File>,
}

pub fn read(input_source: &[PathBuf], _mode: &[Mode]) -> Result<ReadResult, Error> {
    // need judge stdin or current path when vec is empty
    if input_source.is_empty() {
        let stdin = std::io::stdin();

        if stdin.is_terminal() {
            let result = env::current_dir()?
                .read_dir()?
                .filter_map(|s| {
                    if matches!(
                        s.as_ref().ok()?.path().extension().and_then(|s| s.to_str()),
                        Some("pdf" | "epub")
                    ) {
                        None
                    } else {
                        Some(s)
                    }
                })
                .map(|entry| {
                    let path = entry?.path();
                    let file = File::open(&path)?;

                    Ok(ReadF {
                        path,
                        reader: BufReader::new(file),
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;

            Ok(ReadResult::Dir(result))
        } else {
            Ok(ReadResult::Stdin(BufReader::new(stdin)))
        }
    } else if input_source.len() == 1 {
        if input_source[0].is_dir() {
            let result = input_source[0]
                .read_dir()?
                .filter_map(|s| {
                    // remove nonsupport file
                    if matches!(
                        s.as_ref().ok()?.path().extension().and_then(|s| s.to_str()),
                        Some("pdf" | "epub")
                    ) {
                        None
                    } else {
                        Some(s)
                    }
                })
                .map(|entry| {
                    // need recursion ergodic
                    let path = entry?.path();
                    let file = File::open(&path)?;

                    Ok(ReadF {
                        path,
                        reader: BufReader::new(file),
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;

            Ok(ReadResult::Dir(result))
        } else {
            let file = File::open(input_source[0].clone())?;
            Ok(ReadResult::File(ReadF {
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
                Ok(ReadF {
                    path: path.to_path_buf(),
                    reader: BufReader::new(file),
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(ReadResult::Dir(result))
    }
}
