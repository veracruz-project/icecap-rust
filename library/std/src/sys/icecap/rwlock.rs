use super::icecap_impl as imp;

pub struct RWLock(imp::RWLock);

pub type MovableRWLock = RWLock;

unsafe impl Send for RWLock {}
unsafe impl Sync for RWLock {}

impl RWLock {
    pub const fn new() -> RWLock {
        RWLock(imp::RWLock::new())
    }

    #[inline]
    pub unsafe fn read(&self) {
        self.0.read()
    }

    #[inline]
    pub unsafe fn try_read(&self) -> bool {
        self.0.try_read()
    }

    #[inline]
    pub unsafe fn write(&self) {
        self.0.write()
    }

    #[inline]
    pub unsafe fn try_write(&self) -> bool {
        self.0.try_write()
    }

    #[inline]
    pub unsafe fn read_unlock(&self) {
        self.0.read_unlock()
    }

    #[inline]
    pub unsafe fn write_unlock(&self) {
        self.0.write_unlock()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.0.destroy()
    }
}
