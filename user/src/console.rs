use crate::api::*;
use core::fmt::{self, Write};

const STDIN: usize = 0;
const STDOUT: usize = 1;

pub extern "C" fn getchar() -> u8 {
    let mut buf: [u8; 1] = [0; 1];
    read(STDIN, &mut buf);
    buf[0]
}

pub extern "C" fn putchar(c: u8) {
    write(STDOUT, &[c]);
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
