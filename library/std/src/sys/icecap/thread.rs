#![allow(dead_code)]

use crate::ffi::CStr;
use crate::fmt;
use crate::io;
use crate::time::Duration;

pub struct Tid;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Priority(u8);

impl Priority {
    pub const fn into(self) -> u8 {
        self.0
    }

    pub const fn from(x: u8) -> Self {
        Self(x)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub const NORMAL_PRIO: Priority = Priority::from(2);

pub struct Thread;

unsafe impl Send for Thread {}
unsafe impl Sync for Thread {}

pub const DEFAULT_MIN_STACK_SIZE: usize = 0x40000;

impl Thread {
    pub unsafe fn new_with_coreid(
        stack: usize,
        _p: Box<dyn FnOnce()>,
        core_id: isize,
    ) -> io::Result<Thread> {
        println!("Thread::new_with_coreid(stack = 0x{:x}, p = _, core_id = 0x{:x})", stack, core_id);
        Ok(Thread)
    }

    pub unsafe fn new(stack: usize, _p: Box<dyn FnOnce()>) -> io::Result<Thread> {
        println!("Thread::new(stack = 0x{:x}, p = _)", stack);
        Ok(Thread)
    }

    #[inline]
    pub fn yield_now() {
        println!("Thread::yield_now()");
    }

    #[inline]
    pub fn set_name(name: &CStr) {
        println!("Thread::set_name(name = {:?})", name);
    }

    #[inline]
    pub fn sleep(dur: Duration) {
        println!("Thread::sleep(dur = {:?})", dur);
    }

    pub fn join(self) {
        println!("thread.join()");
    }

    #[inline]
    pub fn id(&self) -> Tid {
        println!("thread.id()");
        Tid
    }

    #[inline]
    pub fn into_id(self) -> Tid {
        println!("thread.into_id()");
        Tid
    }
}

pub mod guard {
    pub type Guard = !;
    pub unsafe fn current() -> Option<Guard> {
        None
    }
    pub unsafe fn init() -> Option<Guard> {
        None
    }
}
