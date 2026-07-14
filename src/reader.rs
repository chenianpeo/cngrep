use std::{
    fs::File,
    io::{BufReader, Stdin},
    path::PathBuf,
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

pub fn read(input_source: &Option<PathBuf>, _mode: &Mode) -> Result<ReadResult, Error> {
    match input_source {
        Some(path) if path.is_file() => {
            let file = File::open(path)?;
            // Ok(ReadResult::File(BufReader::new(file)))
            Ok(ReadResult::File(ReadF {
                path: path.to_path_buf(),
                reader: BufReader::new(file),
            }))
        }

        Some(path) if path.is_dir() => {
            let result = path
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
        }

        None => {
            let stdin = std::io::stdin();
            Ok(ReadResult::Stdin(BufReader::new(stdin)))
        }

        _ => Err(Error::Internal {
            context: "read input source".into(),
        }),
    }
}
