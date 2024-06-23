//! Wrap `__switch` as a rust function

use super::TaskContext;
use crate::global_asm;

global_asm!(include_str!("switch.s"));

extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}

// -----------------------------------------------

static mut WAIT_PROC_MANAGER: bool = false;

fn set_proc_manager_service(wait: bool) {
    unsafe {
        WAIT_PROC_MANAGER = wait;
    }
}

pub fn check_proc_manager_service() -> bool {
    unsafe { WAIT_PROC_MANAGER }
}

pub fn set_proc_manager_service_on() {
    set_proc_manager_service(true);
}

pub fn set_proc_manager_service_off() {
    set_proc_manager_service(false);
}
