use crate::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> Result<BufReader<File>, Error> {
    let file = File::open(path)?;
    let file_content = BufReader::new(file);
    Ok(file_content)
}
pub fn read_stdin<'a>() -> Result<std::io::StdinLock<'a>, Error> {
    let buf = std::io::stdin();
    let handle = buf.lock();

    Ok(handle)
}
