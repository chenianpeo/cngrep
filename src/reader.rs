// use std::fs::File;
// use std::io::BufReader;

// use crate::matcher::{NormalFile, Type};

// pub trait Reader: Type {
//     fn read(&self) -> Result<BufReader<File>, crate::error::Error>;
// }

// pub struct FileReader {
//     pub path: PathBuf,
// }

// impl Reader for FileReader {
//     fn read(&self) -> Result<BufReader<File>, crate::error::Error> {
//         let open_file = File::open(&self.path)?;
//         let file_content = BufReader::new(open_file);
//         Ok(file_content)
//     }
// }

// impl Reader for NormalFile {
//     fn read(&self) -> Result<BufReader<File>, crate::error::Error> {
//         let open_file = File::open(&self.file)?;
//         let file_content = BufReader::new(open_file);
//         Ok(file_content)
//     }
// }

// use crate::matcher::Type;

// pub trait Read {
//     fn read(&self);
// }
// impl Read for Type {
//     fn read(&self) {
//         println!("normal file read");
//     }
// }

use crate::matcher::NormalFile;

pub trait Read {
    fn read(&self);
}
impl Read for NormalFile {
    fn read(&self) {
        println!("normal file read");
    }
}