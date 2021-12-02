use crate::sys::mutex::Mutex;
use crate::time::Duration;
use super::icecap_impl as imp;

pub struct Condvar(imp::Condvar);

impl Condvar {
    pub const fn new() -> Condvar {
        Condvar(imp::Condvar::new())
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        self.0.init()
    }

    pub unsafe fn notify_one(&self) {
        self.0.notify_one()
    }

    #[inline]
    pub unsafe fn notify_all(&self) {
        self.0.notify_all()
    }

    pub unsafe fn wait(&self, mutex: &Mutex) {
        self.0.wait(mutex)
    }

    pub unsafe fn wait_timeout(&self, mutex: &Mutex, dur: Duration) -> bool {
        self.0.wait_timeout(mutex, dur)
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.0.destroy()
    }
}
