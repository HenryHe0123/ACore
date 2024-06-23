//! Implementation of [`TaskManager`]
//!
//! A simple FIFO scheduler.

use super::{TaskControlBlock, PROC_MANAGER};
use crate::task::switch::check_proc_manager_service;
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
    if check_proc_manager_service() {
        Some(PROC_MANAGER.clone())
    } else {
        TASK_MANAGER.exclusive_access().fetch()
    }
}

pub fn add_task_front(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add_front(task);
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

    fn add_front(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_front(task);
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> = UPSafeCell::new(TaskManager::new());
}
