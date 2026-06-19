use crate::matcher::NormalFile;

pub trait Read {
    fn read(&self);
}
impl Read for NormalFile {
    fn read(&self) {
        println!("normal file read");
    }
}