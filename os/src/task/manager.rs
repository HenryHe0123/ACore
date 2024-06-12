//! Implementation of [`TaskManager`]
//!
//! A simple FIFO scheduler.

use super::TaskControlBlock;
use crate::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use lazy_static::lazy_static;

/// Interface offered to add task
pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

/// Interface offered to pop the first task
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

// implementation ---------------------------------------------------

/// A queue of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }

    fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = UPSafeCell::new(TaskManager::new());
}
