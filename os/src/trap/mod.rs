//! Trap handling
//!
//! All traps go through a single entry point, `__save_trap_ctx`, which restores the kernel space context
//! and transfers control to `trap_handler()`.
//! `trap_handler()` then calls different functions based on the type of exception.

pub mod context;

use crate::asm;
use crate::config::*;
use crate::syscall::syscall;
use crate::task::*;
use crate::timer::set_next_trigger;
use crate::warn;
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec,
};

global_asm!(include_str!("trampoline.s"));

extern "C" {
    fn __save_trap_ctx();
    fn __restore_ctx();
}

fn set_stvec(addr: usize) {
    unsafe {
        stvec::write(addr, TrapMode::Direct);
    }
}

/// init stvec (trap handler entry)
pub fn init() {
    set_stvec(__save_trap_ctx as usize);
}

fn set_kernel_trap_entry() {
    set_stvec(trap_from_kernel as usize);
}

fn set_user_trap_entry() {
    set_stvec(TRAMPOLINE);
}

#[no_mangle]
/// handle a trap from user space, called by trampoline.s
pub fn trap_handler() -> ! {
    // set stvec to trap_from_kernel
    set_kernel_trap_entry();
    // get current trap context from TASK_MANAGER
    let cx = get_current_trap_cx();
    // get trap cause
    let scause = scause::read();
    // get extra value
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // sepc points to the ecall instruction initially
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            // x10 = a0
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            warn!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return()
}

pub fn trap_return() -> ! {
    // reset stvec to __save_trap_ctx
    set_user_trap_entry();
    // prepare for __restore_ctx
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = get_current_user_token();
    // compute the virtual address of __restore_ctx
    let restore_va = __restore_ctx as usize - __save_trap_ctx as usize + TRAMPOLINE;
    // jump to __restore_ctx
    unsafe {
        asm!(
            "fence.i", // flush I-cache
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    unimplemented!("a trap from kernel!")
}
