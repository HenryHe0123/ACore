mod context;
mod kernel_stack;
mod manager;
mod scheduler;
pub mod service;
pub mod switch;
mod task;

use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;
pub use context::TaskContext;
use lazy_static::lazy_static;
pub use manager::*;
pub use scheduler::*;
use switch::check_proc_manager_service;
use task::*;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap(),
        1
    ));
    pub static ref PROC_MANAGER: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("proc_manager").unwrap(),
        0
    ));
}

pub fn add_initproc() {
    let initproc = INITPROC.clone();
    add_task(initproc);
}

/// Suspend the current 'Running' task and run the next task in task list.
pub fn suspend_current_and_run_next() {
    // There must be an application running.
    let current_task = take_current_task().expect("no current task");
    let mut current_inner = current_task.inner_exclusive_access();
    let task_cx_ptr = current_inner.get_task_cx_ptr();
    drop(current_inner);

    if check_proc_manager_service() {
        add_task_front(current_task);
        // crate::debug!("enter suspend_current_and_run_next: wait proc manager");
    } else {
        // push back to ready queue
        add_task(current_task);
    }

    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

// ---------------------------------------------------------------------

/// Exit the current 'Running' task and run the next task in task list.
pub fn exit_current_and_run_next(exit_code: i32) {
    service::exit(current_pid(), exit_code);

    // take current task from Processor
    let current_task = take_current_task().unwrap();
    // confirm that current task will be deallocated
    if current_task.pid > 1 {
        assert_eq!(Arc::strong_count(&current_task), 1);
    }
    // recycle current task resources
    drop(current_task);

    // we do not have to save task context, just run next
    let mut _unused = TaskContext::empty();
    schedule(&mut _unused as *mut _);
}
