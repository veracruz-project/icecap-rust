use crate::alloc::{GlobalAlloc, Layout, System};

static mut DLMALLOC: dlmalloc::Dlmalloc = dlmalloc::DLMALLOC_INIT;

// TODO sync

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        DLMALLOC.malloc(layout.size(), layout.align())
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        DLMALLOC.calloc(layout.size(), layout.align())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        DLMALLOC.free(ptr, layout.size(), layout.align())
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        DLMALLOC.realloc(ptr, layout.size(), layout.align(), new_size)
    }
}
