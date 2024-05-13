use super::{TaskContext, TaskControlBlock, TaskStatus};
use crate::info;
use crate::loader::*;
use crate::task::switch::__switch;
use crate::UPSafeCell;
use alloc::vec::Vec;

/// where all the tasks are managed
pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    /// task list
    tasks: Vec<TaskControlBlock>,
    /// index of current `Running` task
    current_task: usize,
}

// run_first_task & init ----------------------------

impl TaskManager {
    pub fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::empty();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("Unreachable in run_first_task!");
    }

    pub fn init() -> Self {
        info!("[kernel] init TASK_MANAGER");
        let num_app = get_num_app();
        info!("[kernel] num_app = {}", num_app);
        let mut tasks: Vec<TaskControlBlock> = Vec::new();
        for i in 0..num_app {
            tasks.push(TaskControlBlock::new(get_app_data(i), i));
        }
        TaskManager {
            num_app,
            inner: UPSafeCell::new(TaskManagerInner {
                tasks,
                current_task: 0,
            }),
        }
    }
}

// other methods ------------------------------------

impl TaskManager {
    /// Change the status of current `Running` task into `Ready`.
    pub fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    /// Change the status of current `Running` task into `Exited`.
    pub fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    pub fn run_next_task(&self) {
        unimplemented!()
    }
}
