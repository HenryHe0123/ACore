use crate::info;
use crate::loader::get_app_data_by_name;
use crate::mm::*;
use crate::task::switch::check_proc_manager_service;
use crate::task::*;
use crate::timer::get_time_ms;
use switch::set_proc_manager_service_off;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next(exit_code);
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    if check_proc_manager_service() {
        // yeild from proc_manager, just run next (go back)
        set_proc_manager_service_off();
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
    current_task().unwrap().pid as isize
}

pub fn sys_fork() -> isize {
    let current_task = current_task().unwrap();
    let new_pid = service::fork(current_task.pid); // child process
    let new_task = current_task.fork(new_pid);

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
/// Else return found_pid.
pub fn sys_waitpid(pid: isize, exit_code_ptr: *mut i32) -> isize {
    let (found_pid, exit_code) = service::waitpid(current_pid(), pid);

    match found_pid {
        0 => {
            // no child process
            return -1;
        }
        1 => {
            // child process is still running
            return -2;
        }
        _ => {
            // store exit code to user space
            *translated_refmut(current_user_token(), exit_code_ptr) = exit_code;
            found_pid as isize
        }
    }
}

pub fn sys_shutdown() -> isize {
    info!("[kernel] Shutdown by user.");
    crate::shutdown(false)
}
