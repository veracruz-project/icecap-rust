use crate::io;
use super::icecap_impl::{Void, write_to_fd};

pub struct Stdin(Void);
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Self> {
        unsupported!()
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _data: &mut [u8]) -> io::Result<usize> {
        self.0.void()
    }
}

impl Stdout {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl io::Write for Stdout {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        write_to_fd(1, data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }
}

impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        write_to_fd(2, data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    unsupported!()
}

pub fn panic_output() -> Option<impl io::Write> {
    Stderr::new().ok()
}
