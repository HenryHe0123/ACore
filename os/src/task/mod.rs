mod context;
mod kernel_stack;
mod manager;
mod pid;
pub mod process;
mod processor;
pub mod switch;
mod task;

use crate::loader::get_app_data_by_name;
use alloc::sync::Arc;
pub use context::TaskContext;
use lazy_static::lazy_static;
pub use manager::*;
pub use processor::*;
use switch::check_wait_proc_manager;
use task::*;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
    pub static ref PROC_MANAGER: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("proc_manager").unwrap()
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
    // Change status to Ready
    current_inner.task_status = TaskStatus::Ready;
    drop(current_inner);

    if check_wait_proc_manager() {
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
    let current_task = current_task().expect("no current task");
    let pid = current_task.getpid();
    drop(current_task);
    process::exit_process(pid, exit_code);

    // take current task from Processor
    let current_task = take_current_task().expect("no current task");
    let mut current_inner = current_task.inner_exclusive_access();
    current_inner.task_status = TaskStatus::Zombie;
    // record exit code
    // current_inner.exit_code = exit_code;

    // move its children to initproc
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in current_inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    current_inner.children.clear();

    // deallocate user space
    current_inner.memory_set.recycle_data_pages();
    drop(current_inner);
    // drop task manually to maintain rc correctly
    drop(current_task);
    // we do not have to save task context, just run next
    let mut _unused = TaskContext::empty();
    schedule(&mut _unused as *mut _);
}
