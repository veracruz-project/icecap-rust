#![allow(missing_docs)]

use crate::os::raw::c_char;
use crate::io::ErrorKind;
pub use crate::sys_common::os_str_bytes as os_str;

#[macro_use]
mod impl_local;
use impl_local as icecap_impl;
pub use impl_local::export as icecap_impl_pub;

pub mod alloc;
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fast_thread_local;
pub mod fd;
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod memchr;
pub mod mutex;
#[path = "../unsupported/net.rs"]
pub mod net;
pub mod os;
#[path = "../unix/os_str.rs"]
pub mod os_str;
#[path = "../unix/path.rs"]
pub mod path;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new_const(
        crate::io::ErrorKind::Unsupported,
        &"operation not supported on HermitCore yet",
    )
}

#[cfg(not(test))]
pub fn init(_argc: isize, _argv: *const *const u8) {
}

// NOTE used by both libunwind and libpanic_abort
#[cfg(not(test))]
#[no_mangle]
pub unsafe extern "C" fn __rust_abort() {
    abort_internal();
}

pub unsafe fn abort_internal() -> ! {
    icecap_impl::abort()
}

pub fn decode_error_kind(_errno: i32) -> ErrorKind {
    ErrorKind::Other
}

// TODO
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

pub unsafe fn strlen(s: *const c_char) -> usize {
    let mut i = 0;
    while *s.offset(i) != 0 {
        i += 1;
    }
    return i as usize;
}
