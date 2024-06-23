use crate::config::*;

use crate::task::suspend_current_and_run_next;
use crate::task::switch::set_wait_proc_manager;

// debug: remember to drop execlusive access before switch to proc manager
pub fn switch_to_proc_manager() {
    // crate::debug!("switch_to_proc_manager");
    set_wait_proc_manager(true);
    suspend_current_and_run_next();
}

const EXIT: i32 = 1;
const GET_EXIT_CODE: i32 = 2;

pub fn exit_process(pid: usize, exit_code: i32) {
    // crate::debug!("exit_process: pid = {}, exit_code = {}", pid, exit_code);
    write_to_shared_page(0, EXIT);
    write_to_shared_page(1, pid as i32);
    write_to_shared_page(2, exit_code);
    switch_to_proc_manager();
}

pub fn get_process_exit_code(pid: usize) -> Option<i32> {
    // crate::debug!("get_process_exit_code: pid = {}", pid);
    write_to_shared_page(0, GET_EXIT_CODE);
    write_to_shared_page(1, pid as i32);
    switch_to_proc_manager();

    let exit_code = read_from_shared_page(2);
    if exit_code == -19260817 {
        None
    } else {
        Some(exit_code)
    }
}
