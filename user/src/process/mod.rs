extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::vec::Vec;

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Zombie,
}

pub struct Process {
    pid: usize,
    // state: ProcessState,           // 进程的当前状态（例如 active、dead 等）
    // parent: Option<Weak<Process>>, // 指向父进程的弱引用
    // children: Vec<Arc<Process>>,   // 包含所有子进程的引用
    pub exit_code: i32, // 进程的退出码
}

impl Process {
    pub fn new(pid: usize, exit_code: i32) -> Self {
        Process {
            pid,
            // state: ProcessState::Ready,
            // parent: None,
            // children: Vec::new(),
            exit_code: exit_code,
        }
    }
}

pub struct ProcessManager {
    processes: BTreeMap<usize, Arc<Process>>, // 保存所有进程的引用
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: BTreeMap::new(),
        }
    }

    // pub fn add(&mut self, process: Arc<Process>) {
    //     self.processes.insert(process.pid, process);
    // }

    pub fn remove(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    pub fn get(&self, pid: usize) -> Option<Arc<Process>> {
        self.processes.get(&pid).cloned()
    }

    pub fn get_exit_code(&self, pid: usize) -> Option<i32> {
        self.processes.get(&pid).map(|p| p.exit_code)
    }

    pub fn add_exit_code(&mut self, pid: usize, exit_code: i32) {
        self.processes
            .insert(pid, Arc::new(Process::new(pid, exit_code)));
    }
}
