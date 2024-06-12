mod context;
mod kernel_stack;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

use crate::trap::TrapContext;
use context::TaskContext;
use manager::*;
pub use processor::*;
use task::*;

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let task = take_current_task().expect("no current task");

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    let task_cx_ptr = task_inner.get_task_cx_ptr();
    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    // ---- release current TCB

    // push back to ready queue.
    add_task(task);
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    let task = take_current_task().expect("no current task");

    // ---- access current TCB exclusively
    let mut task_inner = task.inner_exclusive_access();
    task_inner.task_status = TaskStatus::Zombie;
    task_inner.exit_code = exit_code;

    let task_cx_ptr = task_inner.get_task_cx_ptr();
    // Change status to Ready

    drop(task_inner);
    // ---- release current TCB

    unimplemented!()
}

pub fn run_first_task() {
    unimplemented!()
}

/// Get the current 'Running' task's trap context.
pub fn get_current_trap_cx() -> &'static mut TrapContext {
    current_trap_cx()
}

/// Get the current 'Running' task's token (页表地址).
pub fn get_current_user_token() -> usize {
    current_user_token()
}
