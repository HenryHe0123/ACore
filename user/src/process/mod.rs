//! For proc_manager.rs

mod pid;

use crate::up::UPSafeCell;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use pid::*;

pub struct Process {
    pub pid: Pid,
    inner: UPSafeCell<PCBInner>,
}

/// mutable inner data
struct PCBInner {
    pub parent: Option<Weak<Process>>,
    pub children: Vec<Arc<Process>>,
    pub exit_code: i32,
    pub is_zombie: bool,
}

impl PCBInner {
    pub fn new() -> Self {
        PCBInner {
            parent: None,
            children: Vec::new(),
            exit_code: 0,
            is_zombie: false,
        }
    }
}

impl Process {
    pub fn new() -> Self {
        Process {
            pid: pid_alloc(),
            inner: UPSafeCell::new(PCBInner::new()),
        }
    }

    pub fn set_exit_code(&self, exit_code: i32) {
        self.inner.exclusive_access().exit_code = exit_code;
    }

    pub fn get_exit_code(&self) -> i32 {
        self.inner.exclusive_access().exit_code
    }

    pub fn add_child(&self, child: Arc<Process>) {
        self.inner.exclusive_access().children.push(child);
    }

    pub fn set_parent(&self, parent: Weak<Process>) {
        self.inner.exclusive_access().parent = Some(parent);
    }

    pub fn is_zombie(&self) -> bool {
        self.inner.exclusive_access().is_zombie
    }

    pub fn set_zombie(&self) {
        self.inner.exclusive_access().is_zombie = true;
    }
}

// -----------------------------------------------------

pub struct ProcessManager {
    processes: BTreeMap<usize, Arc<Process>>,
}

impl ProcessManager {
    fn new() -> Self {
        ProcessManager {
            processes: BTreeMap::new(),
        }
    }

    fn add(&mut self, process: Arc<Process>) {
        self.processes.insert(process.pid.0, process);
    }

    fn remove(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    fn get(&self, pid: usize) -> Option<Arc<Process>> {
        self.processes.get(&pid).map(|p| p.clone())
    }

    pub fn fork(&mut self, parent_pid: usize) -> usize {
        let parent_process = self.get(parent_pid).unwrap();
        let child_process = Arc::new(Process::new());
        let pid = child_process.pid.0;
        child_process.set_parent(Arc::downgrade(&parent_process));
        parent_process.add_child(child_process.clone());
        self.add(child_process);
        pid
    }

    pub fn exit(&mut self, pid: usize, exit_code: i32) {
        let exit_process = self.get(pid).unwrap();
        exit_process.set_zombie();
        exit_process.set_exit_code(exit_code);
        let mut exit_inner = exit_process.inner.exclusive_access();
        for child in exit_inner.children.iter() {
            child.set_parent(Arc::downgrade(&INITPROC));
            INITPROC.add_child(child.clone());
        }
        exit_inner.children.clear();
    }

    /// If there is not a child process whose pid is same as given, return (0, _).
    /// Else if there is a child process but it is still running, return (1, _).
    /// Else return (found_pid, exit_code).
    pub fn waitpid(&mut self, parent_pid: usize, pid: i32) -> (usize, i32) {
        let parent_process = self.get(parent_pid).unwrap();
        let mut parent_inner = parent_process.inner.exclusive_access();
        let mut found_pid = 0; // assume not found at first
        let mut exit_code = 0;
        let mut remove = false;
        if pid == -1 {
            if parent_inner.children.is_empty() {
                return (0, 0);
            }
            found_pid = 1; // assume found but still running
            for child in parent_inner.children.iter() {
                if child.is_zombie() {
                    found_pid = child.pid.0;
                    exit_code = child.get_exit_code();
                    self.remove(found_pid);
                    remove = true;
                    break;
                }
            }
        } else {
            for child in parent_inner.children.iter() {
                if child.pid.0 == pid as usize {
                    if child.is_zombie() {
                        found_pid = child.pid.0;
                        exit_code = child.get_exit_code();
                        self.remove(found_pid);
                        remove = true;
                    } else {
                        found_pid = 1;
                    }
                    break;
                }
            }
        }
        if remove {
            parent_inner
                .children
                .retain(|child| child.pid.0 != found_pid);
        }
        (found_pid, exit_code)
    }

    pub fn init(&mut self) {
        self.add(Arc::new(Process::new())); // proc manager (pid = 0)
        self.add(INITPROC.clone()); // init process (pid = 1)
    }
}

lazy_static! {
    pub static ref PROC_MANAGER: UPSafeCell<ProcessManager> =
        UPSafeCell::new(ProcessManager::new());
    static ref INITPROC: Arc<Process> = Arc::new(Process::new());
}
