use core::sync::atomic::{AtomicUsize, Ordering};
use crate::hint;

const READER: usize = 1 << 1;
const WRITER: usize = 1 << 0;

pub struct RWLock(AtomicUsize);

impl RWLock {
    pub const fn new() -> Self {
        Self(AtomicUsize::new(0))
    }

    #[inline]
    #[allow(unreachable_code)]
    pub fn read(&self) {
        return; //xx
        while !self.try_read() {
            panic!("rwlock read contention");
            hint::spin_loop()
        }
    }

    #[inline]
    pub fn try_read(&self) -> bool {
        let value = self.0.fetch_add(READER, Ordering::Acquire);
        if value & WRITER != 0 {
            self.0.fetch_sub(READER, Ordering::Release);
            false
        } else {
            true
        }
    }

    #[inline]
    #[allow(unreachable_code)]
    pub fn write(&self) {
        return; //xx
        while !self.try_write_internal(false) {
            panic!("rwlock write contention");
            hint::spin_loop()
        }
    }

    #[inline]
    pub fn try_write(&self) -> bool {
        self.try_write_internal(true)
    }

    #[inline]
    fn try_write_internal(&self, strong: bool) -> bool {
        compare_exchange(
            &self.0,
            0,
            WRITER,
            Ordering::Acquire,
            Ordering::Relaxed,
            strong,
        ).is_ok()
    }

    #[inline]
    pub fn read_unlock(&self) {
        // debug_assert!(self.0.load(Ordering::Relaxed) & !WRITER > 0);
        self.0.fetch_sub(READER, Ordering::Release);
    }

    #[inline]
    pub fn write_unlock(&self) {
        // debug_assert_eq!(self.0.load(Ordering::Relaxed) & !WRITER, 0);
        self.0.fetch_and(!WRITER, Ordering::Release);
    }

    #[inline]
    pub fn destroy(&self) {
    }
}

#[inline(always)]
fn compare_exchange(
    atomic: &AtomicUsize,
    current: usize,
    new: usize,
    success: Ordering,
    failure: Ordering,
    strong: bool,
) -> Result<usize, usize> {
    if strong {
        atomic.compare_exchange(current, new, success, failure)
    } else {
        atomic.compare_exchange_weak(current, new, success, failure)
    }
}
