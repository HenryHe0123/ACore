pub mod uart;

use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

pub fn console_putchar(c: u8) {
    uart::uart_putchar(c);
}

pub fn shutdown(failure: bool) -> ! {
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
