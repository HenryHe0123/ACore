//! Implementation of [`Scheduler`] and Intersection of control flow

use super::{TaskContext, TaskControlBlock};
use crate::info;
use crate::shutdown;
use crate::task::manager::fetch_task;
use crate::task::switch::__switch;
use crate::trap::TrapContext;
use crate::UPSafeCell;
use alloc::sync::Arc;
use lazy_static::lazy_static;

pub struct Scheduler {
    /// The task currently executing on the current processor
    current: Option<Arc<TaskControlBlock>>,
    /// The basic control flow of core, helping to select and switch process
    idle_task_cx: TaskContext,
}

impl Scheduler {
    /// Create an empty Processor
    pub fn new() -> Self {
        Self {
            current: None,
            idle_task_cx: TaskContext::empty(),
        }
    }

    /// Get mutable reference to `idle_task_cx`
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx
    }

    /// Get current task by move Arc (current task will be None)
    fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    /// Get current task by clone Arc
    fn clone_current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

lazy_static! {
    pub static ref SCHEDULER: UPSafeCell<Scheduler> = UPSafeCell::new(Scheduler::new());
}

// interface -------------------------------------------

/// The current task will be None
pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    SCHEDULER.exclusive_access().take_current()
}

/// Get current task by clone Arc
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    SCHEDULER.exclusive_access().clone_current()
}

pub fn current_pid() -> usize {
    current_task().unwrap().pid
}

pub fn current_user_token() -> usize {
    let task = current_task().unwrap();
    let token = task.inner_exclusive_access().get_user_token();
    token
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    current_task()
        .unwrap()
        .inner_exclusive_access()
        .get_trap_cx()
}

// key interface ---------------------------------------

/// The main part of process execution and scheduling
///
/// Loop `fetch_task` to get the process that needs to run, and switch the process through `__switch`
pub fn run_tasks() {
    loop {
        let mut scheduler = SCHEDULER.exclusive_access();
        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = scheduler.get_idle_task_cx_ptr();

            // access coming task TCB exclusively
            let mut task_inner = task.inner_exclusive_access();
            let next_task_cx_ptr = task_inner.get_task_cx_ptr();
            // stop exclusively accessing coming task TCB manually
            drop(task_inner);

            scheduler.current = Some(task);
            drop(scheduler);

            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        } else {
            info!("[kernel] All tasks completed.");
            shutdown(false);
        }
    }
}

/// Return to idle control flow for new scheduling
pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut scheduler = SCHEDULER.exclusive_access();
    let idle_task_cx_ptr = scheduler.get_idle_task_cx_ptr();
    drop(scheduler);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
