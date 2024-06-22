use crate::syscall::*;

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_time()
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

// wait for specific
pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 (error) or a real pid
            exit_pid => return exit_pid,
        }
    }
}

// wait for any
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            exit_pid => return exit_pid,
        }
    }
}

pub fn sleep(period_ms: usize) {
    let start = sys_time();
    while sys_time() < start + period_ms as isize {
        sys_yield();
    }
}

// --------------- for user - kernel communication ----------------------

const SHARED_PAGE: usize = 0x83000000;

pub fn write_to_shared_page(index: usize, value: i32) {
    unsafe {
        let ptr = (SHARED_PAGE as *mut i32).offset(index as isize);
        *ptr = value;
    }
}

pub fn read_from_shared_page(index: usize) -> i32 {
    unsafe {
        let ptr = (SHARED_PAGE as *const i32).offset(index as isize);
        *ptr
    }
}
