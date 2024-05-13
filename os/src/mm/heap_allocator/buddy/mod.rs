pub mod heap;
pub mod list;

use crate::UPSafeCell;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefMut;
use core::ptr::{null_mut, NonNull};

type Heap = heap::MyHeap;

/// My alternative implementation for buddy_system_allocator::LockedHeap
pub struct MyLockedHeap(UPSafeCell<Heap>);

impl MyLockedHeap {
    pub const fn new() -> Self {
        MyLockedHeap(UPSafeCell::new(Heap::new()))
    }

    pub fn lock(&self) -> RefMut<'_, Heap> {
        self.0.exclusive_access()
    }
}

unsafe impl GlobalAlloc for MyLockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock()
            .alloc(layout)
            .ok()
            .map_or(null_mut(), |x| x.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(NonNull::new_unchecked(ptr), layout);
    }
}
