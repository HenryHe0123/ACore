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
use crate::warn;
pub use context::TrapContext;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sip, stval, stvec,
};
use switch::check_proc_manager_service;

global_asm!(include_str!("trampoline.s"));

extern "C" {
    fn __save_trap_ctx();
    fn __restore_ctx();
}

/// init stvec (in kernel)
pub fn init() {
    set_kernel_trap_entry();
}

/// set (s-mode) trap handler entry
fn set_stvec(addr: usize) {
    unsafe {
        stvec::write(addr, TrapMode::Direct);
    }
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
    // get trap cause
    let scause = scause::read();
    // get extra value
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            // get system call return value
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.x[10] = result as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            warn!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, current_trap_cx().sepc);
            // page fault exit code = -2
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("[kernel] IllegalInstruction in application, kernel killed it.");
            // illegal instruction exit code = -3
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            panic!("A strange supervisor timer interrupt occurs! Are you using rustsbi?");
            // set_next_trigger();
            // suspend_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorSoft) => {
            // handle machine timer interrupt delegated from mtvec (timervec)
            let bits = sip::read().bits() & !2; // clear ssip
            unsafe { asm!("csrw sip, {ssip}", ssip = in(reg) bits) };

            // if waiting process manager, ignore timer interrupt
            if !check_proc_manager_service() {
                suspend_current_and_run_next();
            }
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

/// jump to `__restore_ctx` while passing `trap_cx_ptr` and `user_satp`
pub fn trap_return() -> ! {
    // reset stvec to __save_trap_ctx
    set_user_trap_entry();
    // prepare for __restore_ctx
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    // crate::debug!("user token: {:#x}", user_satp);
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
    panic!(
        "Trap when in S-mode! [{:?}] , at address : {:#X}",
        scause::read().cause(),
        stval::read()
    );
}
