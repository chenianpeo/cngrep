use std::fs::{self, File};
use std::io::{self, BufReader};

use crate::config::{CliArgs, ReadResult};
use crate::error::Error;

impl CliArgs {
    pub fn read(&self) -> Result<ReadResult, crate::error::Error> {
        let open_file = File::open(&self.file).map_err(|_| Error::Io {
            source: io::Error::new(io::ErrorKind::NotFound, (self.file).to_string()),
            context: Some("failed to read".to_string()),
        })?;

        let file_content = BufReader::new(open_file);

        let absolute_path = fs::canonicalize(&self.file)?;
        let file_path = absolute_path
            .to_str()
            .ok_or(std::io::Error::other("None"))?
            .to_string();

        Ok(ReadResult {
            path: file_path,
            file: file_content,
        })
    }
}
