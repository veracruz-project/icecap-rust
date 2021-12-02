use crate::ffi::OsString;
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io;
use crate::io::{IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use super::icecap_impl::Void;

pub use crate::sys_common::fs::copy;

#[derive(Debug)]
pub struct File(Void);

pub struct FileAttr(Void);

pub struct ReadDir(Void);

pub struct DirEntry(Void);

#[derive(Clone, Debug)]
pub struct OpenOptions(Void);

pub struct FilePermissions(Void);

pub struct FileType(Void);

#[derive(Debug)]
pub struct DirBuilder(Void);

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0.void()
    }

    pub fn perm(&self) -> FilePermissions {
        self.0.void()
    }

    pub fn file_type(&self) -> FileType {
        self.0.void()
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        self.0.void()
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.0.void()
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        self.0.void()
    }
}

impl Clone for FileAttr {
    fn clone(&self) -> FileAttr {
        self.0.void()
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        self.0.void()
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        self.0.void()
    }
}

impl Clone for FilePermissions {
    fn clone(&self) -> FilePermissions {
        self.0.void()
    }
}

impl PartialEq for FilePermissions {
    fn eq(&self, _other: &FilePermissions) -> bool {
        self.0.void()
    }
}

impl Eq for FilePermissions {}

impl fmt::Debug for FilePermissions {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        self.0.void()
    }

    pub fn is_file(&self) -> bool {
        self.0.void()
    }

    pub fn is_symlink(&self) -> bool {
        self.0.void()
    }
}

impl Clone for FileType {
    fn clone(&self) -> FileType {
        self.0.void()
    }
}

impl Copy for FileType {}

impl PartialEq for FileType {
    fn eq(&self, _other: &FileType) -> bool {
        self.0.void()
    }
}

impl Eq for FileType {}

impl Hash for FileType {
    fn hash<H: Hasher>(&self, _h: &mut H) {
        self.0.void()
    }
}

impl fmt::Debug for FileType {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        self.0.void()
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        self.0.void()
    }

    pub fn file_name(&self) -> OsString {
        self.0.void()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        self.0.void()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        self.0.void()
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        unsupported!()
    }
    pub fn read(&mut self, _read: bool) {
        self.0.void()
    }
    pub fn write(&mut self, _write: bool) {
        self.0.void()
    }
    pub fn append(&mut self, _append: bool) {
        self.0.void()
    }
    pub fn truncate(&mut self, _truncate: bool) {
        self.0.void()
    }
    pub fn create(&mut self, _create: bool) {
        self.0.void()
    }
    pub fn create_new(&mut self, _create_new: bool) {
        self.0.void()
    }
}

impl File {
    pub fn open(_path: &Path, _opts: &OpenOptions) -> io::Result<File> {
        unsupported!()
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        self.0.void()
    }

    pub fn fsync(&self) -> io::Result<()> {
        self.0.void()
    }

    pub fn datasync(&self) -> io::Result<()> {
        self.0.void()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        self.0.void()
    }

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

    pub fn flush(&self) -> io::Result<()> {
        self.0.void()
    }

    pub fn seek(&self, _pos: SeekFrom) -> io::Result<u64> {
        self.0.void()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        self.0.void()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        self.0.void()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        unsupported!()
    }

    pub fn mkdir(&self, _p: &Path) -> io::Result<()> {
        self.0.void()
    }
}

pub fn readdir(_p: &Path) -> io::Result<ReadDir> {
    unsupported!()
}

pub fn unlink(_path: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn rename(_old: &Path, _new: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn set_perm(_p: &Path, _perm: FilePermissions) -> io::Result<()> {
    unsupported!()
}

pub fn rmdir(_p: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn remove_dir_all(_path: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported!()
}

pub fn symlink(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported!()
}

pub fn stat(_p: &Path) -> io::Result<FileAttr> {
    unsupported!()
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported!()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported!()
}
