//! Wrap `__switch` as a rust function

use super::TaskContext;
use crate::global_asm;

global_asm!(include_str!("switch.s"));

extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

// -----------------------------------------------

static mut WAIT_PROC_MANAGER: bool = false;

pub fn check_wait_proc_manager() -> bool {
    unsafe { WAIT_PROC_MANAGER }
}

pub fn set_wait_proc_manager(wait: bool) {
    unsafe {
        WAIT_PROC_MANAGER = wait;
    }
}
