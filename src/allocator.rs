//! Allocator module

use core::alloc::GlobalAlloc;
use core::ptr::null_mut;

#[global_allocator]
static ALLOCATOR: Dummy = Dummy;

/// Dummy allocator
pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        panic!("Not implemented");
    }
}
