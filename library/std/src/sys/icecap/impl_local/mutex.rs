use core::sync::atomic::{AtomicBool, /*AtomicUsize,*/ Ordering};
use crate::hint;

// TODO panic on contention for debugging?

pub struct Mutex(AtomicBool);

impl Mutex {
    pub const fn new() -> Self {
        Self(AtomicBool::new(false))
    }

    #[inline]
    pub fn init(&mut self) {
    }

    #[inline]
    #[allow(unreachable_code)]
    pub fn lock(&self) {
        while !self.0.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_ok() {
            panic!("mutex contention");
            while self.0.load(Ordering::Relaxed) {
                hint::spin_loop();
            }
        }
    }

    #[inline]
    pub fn unlock(&self) {
        self.0.store(false, Ordering::Release);
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        self.0.compare_exchange(false, true, Ordering::Acquire, Ordering::Acquire).is_ok()
    }

    #[inline]
    pub fn destroy(&self) {
    }
}

/*
fn get_tid() -> usize {
    let mut reg;
    unsafe {
        asm!("mrs {}, tpidr_el0", out(reg) reg);
    }
    reg
}
 */

pub struct ReentrantMutex {
    //holder: AtomicUsize,
    //depth: i32,
}

impl ReentrantMutex {
    pub const fn uninitialized() -> Self {
        Self {
            //holder: AtomicUsize::new(0),
            //depth: 0,
        }
    }

    #[inline]
    pub fn init(&self) {
    }

    #[inline]
    #[allow(unreachable_code)]
    pub fn lock(&self) {
        // let tid = get_tid();
        // while !self.0.compare_and_swap(false, true, Ordering::Acquire, Ordering::Acquire).is_ok() {
        //     panic!("mutex contention");
        //     while self.0.load(Ordering::Relaxed) {
        //         hint::spin_loop();
        //     }
        // }
    }

    #[inline]
    pub fn unlock(&self) {
        // self.0.store(false, Ordering::Release);
    }

    #[inline]
    pub fn try_lock(&self) -> bool {
        // self.0.compare_and_swap(false, true, Ordering::Acquire, Ordering::Acquire).is_ok()
        true
    }

    #[inline]
    pub fn destroy(&self) {
    }
}
