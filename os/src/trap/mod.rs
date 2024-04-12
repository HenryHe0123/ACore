pub mod context;

use crate::batch::run_next_app;
use crate::println;
use crate::syscall::syscall;
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trampoline.s"));

/// init stvec (trap handler entry)
pub fn init() {
    extern "C" {
        fn __save_trap_ctx();
    }
    unsafe {
        stvec::write(__save_trap_ctx as usize, TrapMode::Direct);
    }
}

#[no_mangle]
/// handle a trap from user space, called by trap.s
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read(); // get trap cause
    let stval = stval::read(); // get extra value
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // sepc points to the ecall instruction initially
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            // x10 = a0
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
