use core::arch::asm;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],  // a1
            in("x12") args[2],  // a2
            in("x17") id
        );
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_READ: usize = 63;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIME: usize = 169;

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buf.as_mut_ptr() as usize, buf.len()])
}

/// 功能：应用主动交出 CPU 所有权并切换到其他应用。
/// 返回值：总是返回 0。
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_time() -> isize {
    syscall(SYSCALL_TIME, [0, 0, 0])
}
