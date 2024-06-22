extern crate alloc;

use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::vec::Vec;

struct Process {
    pid: usize,
    // state: ProcessState,           // 进程的当前状态（例如 active、dead 等）
    parent: Option<Weak<Process>>, // 指向父进程的弱引用
    children: Vec<Arc<Process>>,   // 包含所有子进程的引用
    exit_code: i32,                // 进程的退出码
}
