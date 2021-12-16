use super::icecap_impl as imp;

pub struct Mutex(imp::Mutex);

pub type MovableMutex = Mutex;

unsafe impl Send for Mutex {}
unsafe impl Sync for Mutex {}

impl Mutex {
    pub const fn new() -> Self {
        Self(imp::Mutex::new())
    }

    #[inline]
    pub unsafe fn init(&mut self) {
        self.0.init()
    }

    #[inline]
    pub unsafe fn lock(&self) {
        self.0.lock()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.0.unlock()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.0.try_lock()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.0.destroy()
    }
}

pub struct ReentrantMutex(imp::ReentrantMutex);

unsafe impl Send for ReentrantMutex {}
unsafe impl Sync for ReentrantMutex {}

impl ReentrantMutex {
    pub const unsafe fn uninitialized() -> Self {
        Self(imp::ReentrantMutex::uninitialized())
    }

    #[inline]
    pub unsafe fn init(&self) {
        self.0.init()
    }

    #[inline]
    pub unsafe fn lock(&self) {
        self.0.lock()
    }

    #[inline]
    pub unsafe fn try_lock(&self) -> bool {
        self.0.try_lock()
    }

    #[inline]
    pub unsafe fn unlock(&self) {
        self.0.unlock()
    }

    #[inline]
    pub unsafe fn destroy(&self) {
        self.0.destroy()
    }
}
