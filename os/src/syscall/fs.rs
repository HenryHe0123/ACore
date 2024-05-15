use crate::mm::page_table::translated_byte_buffer;
use crate::print;
use crate::task::get_current_user_token;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            // debug: user space buffer cannot be accessed directly in kernel space
            let buffers = translated_byte_buffer(get_current_user_token(), buf, len);
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
