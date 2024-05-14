use super::{TaskContext, TaskControlBlock, TaskStatus};
use crate::info;
use crate::loader::*;
use crate::shutdown;
use crate::task::switch::__switch;
use crate::trap::TrapContext;
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
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);
            // before __switch, we should drop local variables manually to release the resources
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            info!("[kernel] All applications completed!");
            shutdown(false);
        }
    }

    /// Find next task to run and return task id.
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        // find the first `Ready` task in task list.
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    pub fn get_current_trap_cx(&self) -> &'static mut TrapContext {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_trap_cx()
    }

    pub fn get_current_token(&self) -> usize {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].get_user_token()
    }
}
