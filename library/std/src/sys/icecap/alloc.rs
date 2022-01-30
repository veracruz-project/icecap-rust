use crate::alloc::{GlobalAlloc, Layout, System};

use icecap_std_impl::dlmalloc::IceCapGlobalAllocator;

static GLOBAL_ALLOCATOR: IceCapGlobalAllocator = IceCapGlobalAllocator;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        GLOBAL_ALLOCATOR.alloc(layout)
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        GLOBAL_ALLOCATOR.alloc_zeroed(layout)
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        GLOBAL_ALLOCATOR.dealloc(ptr, layout)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        GLOBAL_ALLOCATOR.realloc(ptr, layout, new_size)
    }
}
