use crate::mm::address::*;
use crate::sbi::uart::UART0;

pub const VIRT_TEST: usize = 0x10_0000;

const VIRT_MMIO: VARange = VARange {
    start: VirtAddr(VIRT_TEST),
    end: VirtAddr(VIRT_TEST + 0x2000),
};

const UART_MMIO: VARange = VARange {
    start: VirtAddr(UART0),
    end: VirtAddr(UART0 + 0x9000),
};

pub const MMIO: [VARange; 2] = [VIRT_MMIO, UART_MMIO];
