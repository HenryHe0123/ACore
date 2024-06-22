#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8; // Line Feed
const CR: u8 = 0x0du8; // Carriage Return
const DL: u8 = 0x7fu8; // Delete
const BS: u8 = 0x08u8; // Backspace

use alloc::string::String;
use user_lib::console::getchar;
use user_lib::{exec, fork, waitpid};

fn print_prompt() {
    let prompt = "\x1b[32mhenryhe@ACore\x1b[0m:\x1b[34m~\x1b[0m$ ";
    print!("{}", prompt);
}

#[no_mangle]
pub fn main() -> i32 {
    println!("Welcome to Shell!"); // Print welcome message
    let mut line: String = String::new(); // Initialize an empty string to store user input
    print_prompt();

    loop {
        let c = getchar(); // Read a character from user input
        match c {
            LF | CR => {
                // If it's a Line Feed or Carriage Return
                println!(""); // Print a newline (echo the newline)
                if !line.is_empty() {
                    line.push('\0'); // Add a null terminator to the string (C-style string)
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
}
