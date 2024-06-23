#![no_std]
#![no_main]

use user_lib::process::PROC_MANAGER;
use user_lib::read_from_shared_page;
use user_lib::write_to_shared_page;
use user_lib::yield_;

#[macro_use]
extern crate user_lib;

const EXIT: i32 = 1;
const GET_EXIT_CODE: i32 = 2;
const CREATE: i32 = 3;

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
                println!(
                    "[process manager] Process {} exited with code {}",
                    pid, exit_code
                );
                PROC_MANAGER
                    .exclusive_access()
                    .set_exit_code(pid, exit_code);
                yield_();
            }
            GET_EXIT_CODE => {
                let pid = read_from_shared_page(1);
                let exit_code = PROC_MANAGER.exclusive_access().get_exit_code(pid as usize);
                write_to_shared_page(2, exit_code.unwrap_or(0));
                yield_();
            }
            CREATE => {
                let new_pid = PROC_MANAGER.exclusive_access().create_new_process();
                println!("[process manager] Create new process: {}", new_pid);
                write_to_shared_page(1, new_pid as i32);
                yield_();
            }
            _ => {
                panic!("Unknown task: {}", task);
            }
        }
    }
}
