pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod map_area;
pub mod memory_set;
pub mod page_table;

use crate::debug;
use crate::UPSafeCell;
use address::VirtAddr;
use alloc::string::String;
use alloc::sync::Arc;
use lazy_static::lazy_static;
use memory_set::MemorySet;
use page_table::PageTable;

lazy_static! {
    pub static ref KERNEL_SPACE: Arc<UPSafeCell<MemorySet>> =
        Arc::new(UPSafeCell::new(MemorySet::new_kernel()));
}

pub fn init() {
    // heap_allocator::buddy::list::list_test();
    heap_allocator::init_heap();
    // heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    // frame_allocator::frame_allocator_test();
    KERNEL_SPACE.exclusive_access().activate();
}

pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut string = String::new();
    let mut va = ptr as usize;
    loop {
        let ch: u8 = *(page_table
            .translate_va(VirtAddr::from(va))
            .unwrap()
            .get_mut());
        if ch == 0 {
            break;
        } else {
            string.push(ch as char);
            va += 1;
        }
    }
    string
}

/// translate a ptr through page table and return a mutable reference
pub fn translated_refmut<T>(token: usize, ptr: *mut T) -> &'static mut T {
    let page_table = PageTable::from_token(token);
    let va = ptr as usize;
    page_table
        .translate_va(VirtAddr::from(va))
        .unwrap()
        .get_mut()
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
