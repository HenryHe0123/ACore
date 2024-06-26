use crate::mm::address::*;
use crate::sbi::timer::CLINT;
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

const CLINT_MMIO: VARange = VARange {
    start: VirtAddr(CLINT),
    end: VirtAddr(CLINT + 0x10000),
};

pub const MMIO: [VARange; 3] = [VIRT_MMIO, UART_MMIO, CLINT_MMIO];
