use riscv::register::{mie, mstatus};

pub fn init() {
    unsafe {
        // TODO: mtimecmp & mtvec
        // enable m-mode interrupts
        mstatus::set_mie();
        // enable timer interrupt
        mie::set_mtimer();
    }
}
