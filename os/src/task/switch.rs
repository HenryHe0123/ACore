use super::TaskContext;
use crate::global_asm;

global_asm!(include_str!("switch.asm"));

// Wrap switch.asm as a function
extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
