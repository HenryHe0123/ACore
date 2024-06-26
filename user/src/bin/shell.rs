#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;
extern crate user_lib;

const LF: u8 = 0x0au8; // Line Feed
const CR: u8 = 0x0du8; // Carriage Return
const DL: u8 = 0x7fu8; // Delete
const BS: u8 = 0x08u8; // Backspace
const CC: u8 = 3; // Ctrl+C
const ROOT: bool = true;

use alloc::string::String;
use user_lib::console::getchar;
use user_lib::*;

fn print_prompt() {
    let prompt = if ROOT {
        "root@ACore:/# "
    } else {
        "\x1b[32mhenryhe@ACore\x1b[0m:\x1b[34m~\x1b[0m$ "
    };
    print!("{}", prompt);
}

fn check_permission(name: &str) -> bool {
    if name == "proc_manager\0" || name == "initproc\0" || name == "shell\0" {
        println!("Shell: Permission denied!");
        return false;
    } else if name == "shutdown\0" {
        shutdown();
    } else if name == "ls\0" {
        ls();
        return false;
    }
    true
}

#[no_mangle]
pub fn main() -> i32 {
    println!("Welcome to Shell!"); // Print welcome message
                                   // println!("Shell pid = {}", getpid()); // Print the PID of the shell process
    let mut line: String = String::new(); // Initialize an empty string to store user input
    print_prompt();

    loop {
        let c = getchar(); // Read a character from user input
        match c {
            CC => {
                println!("^C");
                break;
            }
            LF | CR => {
                // If it's a Line Feed or Carriage Return
                println!(""); // Print a newline (echo the newline)
                if !line.is_empty() {
                    line.push('\0'); // Add a null terminator to the string (C-style string)

                    if check_permission(line.as_str()) {
                        let pid = fork(); // Create a child process
                        if pid == 0 {
                            // In the child process
                            if exec(line.as_str()) == -1 {
                                println!("Shell: Error when executing {}!", line); // Print error if execution fails
                                return -4; // Return error code
                            }
                            unreachable!();
                        } else {
                            // In the parent process
                            let mut exit_code: i32 = 0;
                            let exit_pid = waitpid(pid as usize, &mut exit_code); // Wait for the child process to finish
                            assert!(pid == exit_pid, "waitpid error"); // Ensure the process waited for is the correct child process
                            println!("Shell: Process {} exited with code {}", pid, exit_code);
                            // Print the exit code of the child process
                        }
                    }
                    line.clear(); // Clear the input line
                }
                print_prompt();
            }
            BS | DL => {
                // If it's a Backspace or Delete
                if !line.is_empty() {
                    // If the input line is not empty
                    print!("{}", BS as char); // Print a backspace character
                    print!(" "); // Print a space character (erase the last character)
                    print!("{}", BS as char); // Print another backspace character (move the cursor back)
                    line.pop(); // Remove the last character from the input line
                }
            }
            _ => {
                // For any other character
                print!("{}", c as char); // Print the character
                line.push(c as char); // Add the character to the input line
            }
        }
    }
    0
}
