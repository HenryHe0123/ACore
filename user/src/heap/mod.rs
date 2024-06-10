pub mod buddy;

use crate::USER_HEAP_SIZE;
use buddy::MyLockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: MyLockedHeap = MyLockedHeap::new();

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
}

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
