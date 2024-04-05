pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod map_area;
pub mod memory_set;
pub mod page_table;

use crate::debug;
use crate::sync::up::UPSafeCell;
use address::VirtAddr;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use memory_set::MemorySet;

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(UPSafeCell::new(MemorySet::new_kernel()));
}

pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

#[allow(unused)]
pub fn remap_test() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }
    let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert_eq!(
        kernel_space
            .page_table
            .translate_to_pte(mid_text.floor())
            .unwrap()
            .writable(),
        false
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_to_pte(mid_rodata.floor())
            .unwrap()
            .writable(),
        false,
    );
    assert_eq!(
        kernel_space
            .page_table
            .translate_to_pte(mid_data.floor())
            .unwrap()
            .executable(),
        false,
    );
    debug!("[test] remap test passed!");
}
