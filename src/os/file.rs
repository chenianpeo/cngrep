use std::ffi::CString;
use std::io;
use std::os::unix::io::RawFd;

use super::syscall::{syscall1, syscall3, syscall4};

// linux x86_64 syscall number
const SYS_READ: usize = 0;

const SYS_CLOSE: usize = 3;

const SYS_OPENAT: usize = 257;

const O_RDONLY: usize = 0;

// current directory
const AT_FDCWD: usize = (-100i64) as usize;

#[derive(Debug)]
pub struct File {
    fd: RawFd,
}

impl File {
    pub fn open(path: &str) -> io::Result<File> {
        let c_path = CString::new(path).unwrap();

        let ret = unsafe { syscall4(SYS_OPENAT, AT_FDCWD, c_path.as_ptr() as usize, O_RDONLY, 0) };

        if ret < 0 {
            Err(io::Error::from_raw_os_error((-ret) as i32))
        } else {
            Ok(File { fd: ret as RawFd })
        }
    }

    pub fn read(&self, buffer: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe {
            syscall3(
                SYS_READ,
                self.fd as usize,
                buffer.as_mut_ptr() as usize,
                buffer.len(),
            )
        };

        if ret < 0 {
            Err(io::Error::from_raw_os_error((-ret) as i32))
        } else {
            Ok(ret as usize)
        }
    }

    pub fn fd(&self) -> RawFd {
        self.fd
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            syscall1(SYS_CLOSE, self.fd as usize);
        }
    }
}
