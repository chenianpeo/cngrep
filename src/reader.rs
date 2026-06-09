use std::fs::{self, File};
use std::io::BufReader;

use crate::config::{CliArgs, ReadResult};
use crate::error::CnError;

impl CliArgs {
    pub fn read(&self) -> Result<ReadResult, CnError> {
        let open_file = File::open(&self.path).unwrap();
        let file_content = BufReader::new(open_file);

        let absolute_path = fs::canonicalize(&self.path).unwrap();
        let file_path = absolute_path.to_str().unwrap().to_string();

        Ok(ReadResult {
            path: file_path,
            file: file_content,
        })
    }
}
