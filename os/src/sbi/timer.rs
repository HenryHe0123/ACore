use crate::config::CLOCK_FREQ;
use riscv::register::{mie, mscratch, mstatus, mtvec};

// clock configuration
pub const MSEC_PER_SEC: usize = 1000; // 1s = 1000ms
const TICKS_PER_SEC: usize = 100; // 100 ticks/s = 10 ms/tick
const INTERVAL: usize = CLOCK_FREQ / TICKS_PER_SEC; // timer interval (clock cycle)

// CLINT : Core Local Interruptor
pub const CLINT: usize = 0x200_0000;
const CLINT_MTIME: usize = CLINT + 0xbff8; // current time
const CLINT_MTIMECMP: usize = CLINT + 0x4000; // next interrupt time

pub fn init() {
    unsafe {
        // use mscratch to pass some data
        // mscratch::write(0);

        // set initial trigger
        set_next_trigger();

        // set mtvec (m-mode trap handler)
        // mtvec::write(0, mtvec::TrapMode::Direct);

        // enable m-mode interrupts
        mstatus::set_mie();
        // enable timer interrupt
        // mie::set_mtimer();
    }
}

/// get current time in clock cycle
pub fn get_time() -> usize {
    unsafe { (CLINT_MTIME as *const usize).read_volatile() }
}

/// get current time in ms
pub fn get_time_ms() -> usize {
    get_time() / (CLOCK_FREQ / MSEC_PER_SEC)
}

fn set_time_cmp(time: usize) {
    unsafe { (CLINT_MTIMECMP as *mut usize).write_volatile(time) }
}

/// set the next timer interrupt
pub fn set_next_trigger() {
    set_time_cmp(get_time() + INTERVAL);
}
