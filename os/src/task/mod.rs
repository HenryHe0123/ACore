mod context;
mod manager;
mod switch;
mod task;

use context::TaskContext;
use task::*;

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
    // TASK_MANAGER.run_first_task();
}

fn mark_current_suspended() {
    // TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    // TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    // TASK_MANAGER.run_next_task();
}
