use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub trait Reader {
    fn read(&self) -> Result<BufReader<File>, crate::error::Error>;
}

pub struct FileReader {
    pub path: PathBuf,
}

impl Reader for FileReader {
    fn read(&self) -> Result<BufReader<File>, crate::error::Error> {
        let open_file = File::open(&self.path)?;
        let file_content = BufReader::new(open_file);
        Ok(file_content)
    }
}
