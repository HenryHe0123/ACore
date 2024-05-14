#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

mod config;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod sync;
mod syscall;
mod task;
mod trap;

#[macro_use]
mod console;

extern crate alloc;

use crate::sbi::{shutdown, timer};
use crate::sync::up::UPSafeCell;
use core::arch::{asm, global_asm};
use riscv::register::{mepc, mstatus, pmpaddr0, pmpcfg0, satp};

global_asm!(include_str!("entry.s"));
global_asm!(include_str!("link_app.s"));

#[no_mangle]
pub fn booting() -> ! {
    clear_bss();
    unsafe {
        // set privilege change to supervisor
        mstatus::set_mpp(mstatus::MPP::Supervisor);
        // set jumping address to kernel_main
        mepc::write(kernel_main as usize);
        // disable page table
        satp::write(0);

        // delegate all interrupt and exception to supervisor
        asm!("csrw mideleg, {}", in(reg) !0);
        asm!("csrw medeleg, {}", in(reg) !0);
        // enable all interrupt
        asm!("csrw sie, {}", in(reg) 0x222);

        // physical memory protection
        pmpaddr0::write(0x3fffffffffffff as usize);
        pmpcfg0::write(0xf);

        // init time interrupt
        timer::init();

        asm!("mret");
    }
    panic!("Unreachable in booting!");
}

#[no_mangle]
pub fn kernel_main() -> ! {
    print_init_info();
    mm::init();
    info!("[kernel] Hello, MMU!");
    mm::remap_test();
    trap::init();
    timer::set_next_trigger();
    task::run_first_task();
    unreachable!()
}

fn print_init_info() {
    info!("{}", sbi::LOGO);
    info!("[mysbi] Hello, kernel!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|p| unsafe { (p as *mut u8).write_volatile(0) });
}
