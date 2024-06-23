#![no_std]
#![no_main]

use user_lib::process::PROC_MANAGER;
use user_lib::read_from_shared_page;
use user_lib::write_to_shared_page;
use user_lib::yield_;

#[macro_use]
extern crate user_lib;

const EXIT: i32 = 1;
const WAIT: i32 = 2;
const FORK: i32 = 3;
const DEBUG: bool = false;

fn init_proc_manager() {
    PROC_MANAGER.exclusive_access().init();
}

#[no_mangle]
fn main() {
    println!("[process manager] Start running.");
    init_proc_manager();
    loop {
        let task = read_from_shared_page(0);
        // println!("[process manager] Handle Process Task: {}", task);
        match task {
            EXIT => {
                let pid = read_from_shared_page(1) as usize;
                let exit_code = read_from_shared_page(2);
                PROC_MANAGER.exclusive_access().exit(pid, exit_code);
                if DEBUG {
                    println!("[process manager] Exit process: {}", pid);
                }
                yield_();
            }
            WAIT => {
                let parent_pid = read_from_shared_page(1) as usize;
                let pid = read_from_shared_page(2);
                let (found_pid, exit_code) =
                    PROC_MANAGER.exclusive_access().waitpid(parent_pid, pid);
                write_to_shared_page(3, found_pid as i32);
                write_to_shared_page(4, exit_code);
                yield_();
            }
            FORK => {
                let parent_pid = read_from_shared_page(1) as usize;
                let new_pid = PROC_MANAGER.exclusive_access().fork(parent_pid);
                write_to_shared_page(2, new_pid as i32);
                if DEBUG {
                    println!("[process manager] Fork new process: {}", new_pid);
                }
                yield_();
            }
            _ => {
                panic!("Unknown task: {}", task);
            }
        }
    }
}
