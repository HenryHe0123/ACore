#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

pub mod api;
mod lang_items;
mod syscall;

#[macro_use]
pub mod console;

pub use api::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
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
