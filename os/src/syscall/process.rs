use crate::info;
use crate::loader::get_app_data_by_name;
use crate::mm::*;
use crate::task::switch::check_wait_proc_manager;
use crate::task::*;
use crate::timer::get_time_ms;
use alloc::sync::Arc;
use switch::set_wait_proc_manager;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    if check_wait_proc_manager() {
        // yeild from proc_manager, just run next (go back)
        set_wait_proc_manager(false);
        let mut _unused = TaskContext::empty();
        schedule(&mut _unused as *mut _);
    } else {
        suspend_current_and_run_next();
    }
    0
}

/// get current time in ms
pub fn sys_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().pid.0 as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_task = current_task.fork(); // child process
    let new_pid = new_task.pid.0;

    // for child process, fork returns 0 to u-mode when it's scheduled
    // so modify trap context of new_task, it will not go back to trap_return
    let trap_cx = new_task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    trap_cx.x[10] = 0; // x[10] is a0 reg
    add_task(new_task); // add child process to scheduler

    new_pid as isize // for parent process, fork returns pid of child process
}

pub fn sys_exec(path: *const u8) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
    // crate::debug!("sys_exec: path = {:?}", path);
    if let Some(data) = get_app_data_by_name(path.as_str()) {
        let task = current_task().unwrap();
        task.exec(data);
        0
    } else {
        -1
    }
}

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let task = current_task().expect("no current task");

    // find a child process
    let mut inner = task.inner_exclusive_access();
    if inner
        .children
        .iter()
        .find(|p| pid == -1 || pid as usize == p.getpid())
        .is_none()
    {
        return -1;
    }

    let pair = inner.children.iter().enumerate().find(|(_, p)| {
        p.inner_exclusive_access().is_zombie() && (pid == -1 || pid as usize == p.getpid())
    });

    if let Some((idx, _)) = pair {
        // find zombie child process
        let child = inner.children.remove(idx);
        // confirm that child will be deallocated after removing from children list
        assert_eq!(Arc::strong_count(&child), 1);
        let found_pid = child.getpid();
        drop(inner);

        let exit_code = process::get_process_exit_code(found_pid).unwrap();

        let inner = task.inner_exclusive_access();

        // store exit code to user space
        *translated_refmut(inner.memory_set.satp_token(), exit_code_ptr) = exit_code;
        found_pid as isize
    } else {
        -2
    }
}
