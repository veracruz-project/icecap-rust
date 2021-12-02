#![allow(missing_docs)]

use crate::os::raw::c_char;
use crate::io::ErrorKind;

#[macro_use]
mod impl_local;
use impl_local as icecap_impl;
pub use impl_local::export as icecap_impl_pub;

pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unix/cmath.rs"]
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fast_thread_local;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
pub mod memchr;
pub mod mutex;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
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

#[path = "../unsupported/common.rs"]
pub mod unsupported_common;

pub use unsupported_common::{
    unsupported, init, cleanup, decode_error_kind, hashmap_random_keys, strlen,
};

// TODO
// NOTE used by both libunwind and libpanic_abort
// #[cfg(not(test))]
// #[no_mangle]
// pub unsafe extern "C" fn __rust_abort() {
//     abort_internal();
// }

pub unsafe fn abort_internal() -> ! {
    icecap_impl::abort()
}
