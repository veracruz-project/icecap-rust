extern crate icecap_std_impl;

mod void;
mod mutex;
mod rwlock;
mod condvar;

pub use void::Void;
pub use mutex::{Mutex, ReentrantMutex};
pub use rwlock::RWLock;
pub use condvar::Condvar;

pub use icecap_std_impl::*;

#[stable(feature = "icecap", since = "0")]
pub mod export {
    #[stable(feature = "icecap", since = "0")]
    pub use super::{
        set_now,
    };
    #[stable(feature = "icecap", since = "0")]
    pub use icecap_std_impl as external;
}

macro_rules! unsupported {
    () => (panic!("not supported"));
    ($($arg:tt)+) => (panic!("not supported: {}", $crate::format_args!($($arg)+)));
}

