// 16550 UART (simulated by qemu)
#![allow(unused)]

const UART0: usize = 0x10000000; // base address of the UART

// UART control registers
const RHR: usize = 0; // receive holding register (for input bytes)
const THR: usize = 0; // transmit holding register (for output bytes)
const IER: usize = 1; // interrupt enable register
const IER_RX_ENABLE: u8 = 1 << 0; // enable receiver interrupt
const IER_TX_ENABLE: u8 = 1 << 1; // enable transmitter interrupt
const FCR: usize = 2; // FIFO control register
const FCR_FIFO_ENABLE: u8 = 1 << 0; // enable FIFOs
const FCR_FIFO_CLEAR: u8 = 3 << 1; // clear FIFOs
const ISR: usize = 2; // interrupt status register
const LCR: usize = 3; // line control register
const LCR_EIGHT_BITS: u8 = 3 << 0; // eight bits
const LCR_BAUD_LATCH: u8 = 1 << 7; // special mode to set baud rate
const LSR: usize = 5; // line status register
const LSR_RX_READY: u8 = 1 << 0; // input is waiting to be read from RHR
const LSR_TX_IDLE: u8 = 1 << 5; // THR can accept another character to send

fn reg(reg_offset: usize) -> *mut u8 {
    unsafe { (UART0 + reg_offset) as *mut u8 }
}

fn read_reg(reg_offset: usize) -> u8 {
    unsafe { reg(reg_offset).read_volatile() }
}

fn write_reg(reg_offset: usize, data: u8) {
    unsafe { reg(reg_offset).write_volatile(data) }
}

pub fn uart_putchar(c: u8) {
    while read_reg(LSR) & LSR_TX_IDLE == 0 {}
    write_reg(THR, c);
}

// return None if no input data
pub fn uart_getchar() -> Option<u8> {
    if read_reg(LSR) & LSR_RX_READY != 0 {
        Some(read_reg(RHR))
    } else {
        None
    }
}

pub fn uart_init() {
    // disable interrupts
    write_reg(IER, 0);
    // set divisor latch (DLAB bit in LCR)
    write_reg(LCR, LCR_BAUD_LATCH);
    // LSB for baud rate of 38.4K
    write_reg(0, 3);
    // MSB for baud rate of 38.4K
    write_reg(1, 0);
    // 8 data bits, 1 stop bit, no parity
    write_reg(LCR, LCR_EIGHT_BITS);
    // reset and enable FIFOs
    write_reg(FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
    // enable receiver buffer interrupts
    write_reg(IER, IER_RX_ENABLE);
}
