use crate::io::{self, IoSlice, IoSliceMut};
use super::icecap_impl::Void;

pub struct AnonPipe(Void);

impl AnonPipe {
    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        self.0.void()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.void()
    }
}

pub fn read2(_p1: AnonPipe, _v1: &mut Vec<u8>, _p2: AnonPipe, _v2: &mut Vec<u8>) -> io::Result<()> {
    unsupported!()
}
