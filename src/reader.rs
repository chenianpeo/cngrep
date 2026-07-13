use std::{
    fs::File,
    io::{BufReader, Stdin},
    path::PathBuf,
};

use crate::{cli::Mode, error::Error};

#[derive(Debug)]
pub enum ReadResult {
    Stdin(BufReader<Stdin>),
    Dir(Vec<BufReader<File>>),
    File(BufReader<File>),
}

pub fn read(input_source: &Option<PathBuf>, _mode: &Mode) -> Result<ReadResult, Error> {
    match input_source {
        Some(path) if path.is_file() => {
            let file = File::open(path)?;
            Ok(ReadResult::File(BufReader::new(file)))
        }

        Some(path) if path.is_dir() => {
            let result = path
                .read_dir()?
                .map(|entry| {
                    let file = File::open(entry?.path())?;
                    Ok(BufReader::new(file))
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
