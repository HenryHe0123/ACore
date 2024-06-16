mod context;
mod kernel_stack;
mod manager;
mod pid;
mod processor;
mod switch;
mod task;

use crate::debug;
use crate::loader::get_app_data_by_name;
use crate::println;
use crate::shutdown;
use alloc::sync::Arc;
use context::TaskContext;
use lazy_static::lazy_static;
pub use manager::*;
pub use processor::*;
use task::*;

lazy_static! {
    pub static ref INITPROC: Arc<TaskControlBlock> = Arc::new(TaskControlBlock::new(
        get_app_data_by_name("initproc").unwrap()
    ));
}

pub fn add_initproc() {
    let initproc = INITPROC.clone();
    add_task(initproc);
}

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
    // take from Processor
    let task = take_current_task().expect("no current task");

    // ---- access current TCB exclusively
    let mut inner = task.inner_exclusive_access();
    inner.task_status = TaskStatus::Zombie;
    // record exit code
    inner.exit_code = exit_code;

    // move its children to initproc
    // ++++++ access initproc TCB exclusively
    {
        let mut initproc_inner = INITPROC.inner_exclusive_access();
        for child in inner.children.iter() {
            child.inner_exclusive_access().parent = Some(Arc::downgrade(&INITPROC));
            initproc_inner.children.push(child.clone());
        }
    }
    inner.children.clear();

    // deallocate user space
    inner.memory_set.recycle_data_pages();
    drop(inner);
    // ---- stop exclusively accessing current PCB
    // drop task manually to maintain rc correctly
    drop(task);
    // we do not have to save task context, just run next
    let mut _unused = TaskContext::empty();
    schedule(&mut _unused as *mut _);
}
