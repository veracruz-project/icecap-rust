use crate::io;
use super::icecap_impl::write_to_fd;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Self {
        Self
    }
}

impl io::Read for Stdin {
    fn read(&mut self, _data: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub const fn new() -> Self {
        Self
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
    pub const fn new() -> Self {
        Self
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
    Some(Stderr::new())
}
