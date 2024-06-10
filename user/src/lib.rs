#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

pub mod api;
mod heap;
mod lang_items;
mod syscall;
mod up;

#[macro_use]
pub mod console;

use crate::heap::init_heap;
pub use api::*;

const USER_HEAP_SIZE: usize = 16384;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    init_heap();
    exit(main());
    panic!("Unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|p| unsafe { (p as *mut u8).write_volatile(0) });
}
