use crate::asm;
use crate::config::CLOCK_FREQ;
use core::ptr::addr_of;
use riscv::register::{mie, mscratch, mstatus, mtvec};

// clock configuration
pub const MSEC_PER_SEC: usize = 1000; // 1s = 1000ms
const TICKS_PER_SEC: usize = 100; // 100 ticks/s = 10 ms/tick
const INTERVAL: usize = CLOCK_FREQ / TICKS_PER_SEC; // timer interval (clock cycle)

// CLINT : Core Local Interruptor
pub const CLINT: usize = 0x200_0000;
const CLINT_MTIME: usize = CLINT + 0xbff8; // current time
const CLINT_MTIMECMP: usize = CLINT + 0x4000; // next interrupt time

static mut TEMP: [usize; 4] = [0; 4];

pub fn init() {
    unsafe {
        // use mscratch to pass some data
        mscratch::write(addr_of!(TEMP) as usize);

        // set initial trigger
        set_next_trigger();

        // set mtvec (m-mode trap handler)
        mtvec::write(timervec as usize, mtvec::TrapMode::Direct);

        // enable m-mode interrupts
        mstatus::set_mie();
        // enable timer interrupt
        mie::set_mtimer();
    }
}

/// delegate MTI to supervisor software interrupt
///
/// #\[naked\] cannot be removed for some reasons
#[naked]
#[no_mangle]
pub extern "C" fn timervec() -> ! {
    unsafe {
        asm!(r#"
        .align 4
        
        # store registers
        csrrw sp, mscratch, sp
        sd a0, 0(sp)
        sd a1, 8(sp)
        sd a2, 16(sp)

        # set next time
        li a0, {mtimecmp}
        ld a1, 0(a0) # a1 = mtimecmp
        li a2, {interval}
        add a1, a1, a2
        sd a1, 0(a0)

        # delegate to a supervisor software interrupt like xv6
        li a0, 2
        csrrs zero, mip, a0

        # restore
        ld a0, 0(sp)
        ld a1, 8(sp)
        ld a2, 16(sp)
        csrrw sp, mscratch, sp

        mret
        "#, 
        mtimecmp = const CLINT_MTIMECMP,
        interval = const INTERVAL,
        options(noreturn))
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

/// set next timer interrupt
pub fn set_next_trigger() {
    set_time_cmp(get_time() + INTERVAL);
}
