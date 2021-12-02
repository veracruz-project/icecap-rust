use crate::error::Error as StdError;
use crate::ffi::{OsStr, OsString};
use crate::fmt;
use crate::io;
use crate::path::{self, PathBuf};
use super::icecap_impl::Void;

pub fn errno() -> i32 {
    unsupported!()
}

pub fn error_string(_errno: i32) -> String {
    unsupported!()
}

pub fn getcwd() -> io::Result<PathBuf> {
    unsupported!()
}

pub fn chdir(_: &path::Path) -> io::Result<()> {
    unsupported!()
}

pub struct SplitPaths<'a>(&'a Void);

pub fn split_paths(_unparsed: &OsStr) -> SplitPaths<'_> {
    unsupported!()
}

impl<'a> Iterator for SplitPaths<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<PathBuf> {
        self.0.void()
    }
}

#[derive(Debug)]
pub struct JoinPathsError(Void);

pub fn join_paths<I, T>(_paths: I) -> Result<OsString, JoinPathsError>
where
    I: Iterator<Item = T>,
    T: AsRef<OsStr>,
{
    unsupported!()
}

impl fmt::Display for JoinPathsError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

impl StdError for JoinPathsError {
}

pub fn current_exe() -> io::Result<PathBuf> {
    unsupported!()
}

pub struct Env;

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<(OsString, OsString)> {
        None
    }
}

pub fn env() -> Env {
    Env
}

pub fn getenv(_k: &OsStr) -> io::Result<Option<OsString>> {
    Ok(None)
}

pub fn setenv(_k: &OsStr, _v: &OsStr) -> io::Result<()> {
    unsupported!()
}

pub fn unsetenv(_k: &OsStr) -> io::Result<()> {
    unsupported!()
}

pub fn temp_dir() -> PathBuf {
    unsupported!()
}

pub fn home_dir() -> Option<PathBuf> {
    unsupported!()
}

pub fn exit(_code: i32) -> ! {
    unsupported!()
}

pub fn getpid() -> u32 {
    unsupported!()
}
