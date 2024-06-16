use crate::UPSafeCell;
use alloc::vec::Vec;
use lazy_static::lazy_static;

pub struct Pid(pub usize);

struct PidAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl PidAllocator {
    fn new() -> Self {
        PidAllocator {
            current: 1,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Pid {
        if let Some(pid) = self.recycled.pop() {
            Pid(pid)
        } else {
            self.current += 1;
            Pid(self.current - 1)
        }
    }

    fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(
            self.recycled.iter().find(|ppid| **ppid == pid).is_none(),
            "pid {} has been deallocated!",
            pid
        );
        self.recycled.push(pid);
    }
}

lazy_static! {
    static ref PID_ALLOCATOR: UPSafeCell<PidAllocator> = UPSafeCell::new(PidAllocator::new());
}

// RAII
impl Drop for Pid {
    fn drop(&mut self) {
        PID_ALLOCATOR.exclusive_access().dealloc(self.0);
    }
}

pub fn pid_alloc() -> Pid {
    PID_ALLOCATOR.exclusive_access().alloc()
}
