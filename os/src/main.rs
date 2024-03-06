#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("link_app.s"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    //panic!("test panic");
    sbi::shutdown(false);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|p| unsafe { (p as *mut u8).write_volatile(0) });
}
