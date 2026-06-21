use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// read file and stdin content
// need different approach for each type
pub fn read_file(path: PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path).map_err(|_| Error::Io {
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "not found target file"),
        context: Some("file read error".to_string()),
    })?;
    let file_content = BufReader::new(file);
    Ok(file_content)
}

pub fn read_stdin<'a>() -> Result<std::io::StdinLock<'a>, Error> {
    let buf = std::io::stdin();
    let handle = buf.lock();

    Ok(handle)
}
