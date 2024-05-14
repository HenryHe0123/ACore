//! Wrap `__switch` as a rust function

use super::TaskContext;
use crate::global_asm;

global_asm!(include_str!("switch.s"));

extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
