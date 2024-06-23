use crate::loader::ls;
use crate::mm::page_table::translated_byte_buffer;
use crate::print;
use crate::sbi::console_getchar;
use crate::task::*;
use alloc::vec::Vec;

const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;

fn translate_buffer(buf: *const u8, len: usize) -> Vec<&'static mut [u8]> {
    translated_byte_buffer(current_user_token(), buf, len)
}

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // debug: user space buffer cannot be accessed directly in kernel space
            let buffers = translate_buffer(buf, len);
            for buffer in buffers {
                let str = core::str::from_utf8(buffer).unwrap();
                print!("{}", str);
            }
            len as isize
        }
        _ => {
            panic!("Unsupported fd {} in sys_write!", fd);
        }
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDIN => {
            assert_eq!(len, 1, "Only support len = 1 in sys_read!");
            let mut c: u8;
            loop {
                c = console_getchar();
                if c == 0 {
                    suspend_current_and_run_next();
                    continue;
                } else {
                    break;
                }
            }
            let mut buffers = translate_buffer(buf, len);
            unsafe {
                buffers[0].as_mut_ptr().write_volatile(c);
            }
            1
        }
        _ => {
            panic!("Unsupported fd in sys_read!");
        }
    }
}

pub fn sys_ls() -> isize {
    ls();
    0
}
