use crate::error;
use crate::info;

pub mod mmio;
pub mod timer;
pub mod uart;

pub fn console_putchar(c: u8) {
    uart::uart_putchar(c);
}

const VIRT_TEST: *mut u32 = mmio::VIRT_TEST as *mut u32;
const TEST_PASS: u32 = 0x5555;

pub fn shutdown(failure: bool) -> ! {
    if failure {
        error!("[mysbi] Error exiting OS!");
    } else {
        info!("[mysbi] Normal shutdown...")
    }
    unsafe { VIRT_TEST.write_volatile(TEST_PASS) };
    unreachable!()
}

pub const LOGO: &str = r" ___  ___  ____    ____  _______  ______    __  
|   \/   | \   \  /   / /       ||   _  \  |  | 
|  \  /  |  \   \/   / |   (----`|  |_)  | |  | 
|  |\/|  |   \_    _/   \   \    |   _  <  |  | 
|  |  |  |     |  | .----)   |   |  |_)  | |  | 
|__|  |__|     |__| |_______/    |______/  |__|";
