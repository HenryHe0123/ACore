pub mod timer;
pub mod uart;

pub fn console_putchar(c: u8) {
    uart::uart_putchar(c);
}

const VIRT_TEST: *mut u32 = 0x100000 as *mut u32;
const TEST_PASS: u32 = 0x5555;
const TEST_FAIL: u32 = 0x3333;

pub fn shutdown(failure: bool) -> ! {
    let value = if failure { TEST_FAIL } else { TEST_PASS };
    unsafe { VIRT_TEST.write_volatile(value) };
    unreachable!()
}

pub const LOGO: &str = r" ___  ___  ____    ____  _______  ______    __  
|   \/   | \   \  /   / /       ||   _  \  |  | 
|  \  /  |  \   \/   / |   (----`|  |_)  | |  | 
|  |\/|  |   \_    _/   \   \    |   _  <  |  | 
|  |  |  |     |  | .----)   |   |  |_)  | |  | 
|__|  |__|     |__| |_______/    |______/  |__|";
