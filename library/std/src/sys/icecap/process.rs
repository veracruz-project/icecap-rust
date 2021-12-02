use crate::ffi::OsStr;
use crate::fmt;
use crate::io;
use crate::sys::fs::File;
use crate::sys::pipe::AnonPipe;
use crate::sys_common::process::CommandEnv;
use super::icecap_impl::Void;

pub use crate::ffi::OsString as EnvKey;

pub struct Command(Void);

pub struct StdioPipes {
    pub stdin: Option<AnonPipe>,
    pub stdout: Option<AnonPipe>,
    pub stderr: Option<AnonPipe>,
}

pub enum Stdio {
    Inherit,
    Null,
    MakePipe,
}

impl Command {
    pub fn new(_program: &OsStr) -> Command {
        unsupported!()
    }

    pub fn arg(&mut self, _arg: &OsStr) {
        self.0.void()
    }

    pub fn env_mut(&mut self) -> &mut CommandEnv {
        self.0.void()
    }

    pub fn cwd(&mut self, _dir: &OsStr) {
        self.0.void()
    }

    pub fn stdin(&mut self, _stdin: Stdio) {
        self.0.void()
    }

    pub fn stdout(&mut self, _stdout: Stdio) {
        self.0.void()
    }

    pub fn stderr(&mut self, _stderr: Stdio) {
        self.0.void()
    }

    pub fn spawn(&mut self, _default: Stdio, _needs_stdin: bool) -> io::Result<(Process, StdioPipes)> {
        self.0.void()
    }
}

impl From<AnonPipe> for Stdio {
    fn from(_pipe: AnonPipe) -> Stdio {
        unsupported!()
    }
}

impl From<File> for Stdio {
    fn from(_file: File) -> Stdio {
        unsupported!()
    }
}

impl fmt::Debug for Command {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

pub struct ExitStatus(Void);

impl ExitStatus {
    pub fn success(&self) -> bool {
        self.0.void()
    }

    pub fn code(&self) -> Option<i32> {
        self.0.void()
    }
}

impl Clone for ExitStatus {
    fn clone(&self) -> ExitStatus {
        self.0.void()
    }
}

impl Copy for ExitStatus {}

impl PartialEq for ExitStatus {
    fn eq(&self, _other: &ExitStatus) -> bool {
        self.0.void()
    }
}

impl Eq for ExitStatus {}

impl fmt::Debug for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

impl fmt::Display for ExitStatus {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.void()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ExitCode;

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode;
    pub const FAILURE: ExitCode = ExitCode;

    pub fn as_i32(&self) -> i32 {
        0
    }
}

pub struct Process(Void);

impl Process {
    pub fn id(&self) -> u32 {
        self.0.void()
    }

    pub fn kill(&mut self) -> io::Result<()> {
        self.0.void()
    }

    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        self.0.void()
    }

    pub fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.0.void()
    }
}
