#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::*;

#[no_mangle]
fn main() -> i32 {
    println!("[initproc] Start running.");
    if fork() == 0 {
        exec("shell\0");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                println!("[initproc] No child process left, exiting...");
                exit(0);
            }
        }
    }
    0
}
