#![unstable(reason = "not public", issue = "none", feature = "fd")]

use crate::io::{self, Read};
use crate::sys_common::AsInner;
use super::icecap_impl::Void;

#[derive(Debug)]
pub struct FileDesc(Void);

impl FileDesc {
    pub fn new(_fd: i32) -> FileDesc {
        unsupported!()
    }

    pub fn raw(&self) -> i32 {
        self.0.void()
    }

    pub fn into_raw(self) -> i32 {
        self.0.void()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn read_to_end(&self, _buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.void()
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn duplicate(&self) -> io::Result<FileDesc> {
        self.0.void()
    }

    pub fn duplicate_path(&self, _path: &[u8]) -> io::Result<FileDesc> {
        self.0.void()
    }

    pub fn nonblocking(&self) -> io::Result<bool> {
        self.0.void()
    }

    pub fn set_cloexec(&self) -> io::Result<()> {
        self.0.void()
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
        self.0.void()
    }
}

impl<'a> Read for &'a FileDesc {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }
}

impl AsInner<i32> for FileDesc {
    fn as_inner(&self) -> &i32 {
        self.0.void()
    }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        self.0.void()
    }
}
