mod context;
mod kernel_stack;
mod manager;
mod pid;
mod switch;
mod task;

use crate::trap::TrapContext;
use context::TaskContext;
use lazy_static::lazy_static;
use manager::TaskManager;
use task::*;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager::init();
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

/// Get the current 'Running' task's trap context.
pub fn get_current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}

/// Get the current 'Running' task's token (页表地址).
pub fn get_current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}
