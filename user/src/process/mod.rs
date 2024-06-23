//! For process manager

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
    exit_code: i32,
}

impl Process {
    pub fn new() -> Self {
        Process {
            pid: pid_alloc(),
            exit_code: 0,
        }
    }

    pub fn set_exit_code(&mut self, exit_code: i32) {
        self.exit_code = exit_code;
    }
}

pub struct ProcessManager {
    processes: BTreeMap<usize, Process>,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            processes: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, process: Process) {
        self.processes.insert(process.pid.0, process);
    }

    pub fn remove(&mut self, pid: usize) {
        self.processes.remove(&pid);
    }

    pub fn get(&self, pid: usize) -> Option<&Process> {
        self.processes.get(&pid)
    }

    pub fn get_mut(&mut self, pid: usize) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }

    pub fn set_exit_code(&mut self, pid: usize, exit_code: i32) {
        if let Some(process) = self.processes.get_mut(&pid) {
            process.set_exit_code(exit_code);
        }
    }

    pub fn get_exit_code(&self, pid: usize) -> Option<i32> {
        self.processes.get(&pid).map(|p| p.exit_code)
    }

    pub fn create_new_process(&mut self) -> usize {
        let process = Process::new();
        let pid = process.pid.0;
        self.add(process);
        pid
    }

    pub fn init(&mut self) {
        self.add(Process::new()); // init process
        self.add(Process::new()); // process manager
    }
}

lazy_static! {
    pub static ref PROC_MANAGER: UPSafeCell<ProcessManager> =
        UPSafeCell::new(ProcessManager::new());
}
