use crate::config::*;
use crate::task::switch::switch_to_proc_manager;

const EXIT: i32 = 1;
const WAIT: i32 = 2;
const FORK: i32 = 3;

pub fn exit(pid: usize, exit_code: i32) {
    write_to_shared_page(0, EXIT);
    write_to_shared_page(1, pid as i32);
    write_to_shared_page(2, exit_code);
    switch_to_proc_manager();
}

/// If there is not a child process whose pid is same as given, return (0, _).
/// Else if there is a child process but it is still running, return (1, _).
/// Else return (found_pid, exit_code).
pub fn waitpid(parent_pid: usize, pid: isize) -> (usize, i32) {
    write_to_shared_page(0, WAIT);
    write_to_shared_page(1, parent_pid as i32);
    write_to_shared_page(2, pid as i32);
    switch_to_proc_manager();

    let found_pid = read_from_shared_page(3);
    let exit_code = read_from_shared_page(4);
    (found_pid as usize, exit_code)
}

pub fn fork(parent_pid: usize) -> usize {
    write_to_shared_page(0, FORK);
    write_to_shared_page(1, parent_pid as i32);
    switch_to_proc_manager();

    let new_pid = read_from_shared_page(2);
    new_pid as usize
}
