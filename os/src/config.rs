pub const USER_STACK_SIZE: usize = 4096 * 2; // 8KB
pub const KERNEL_STACK_SIZE: usize = 4096 * 2; // 8KB
pub const KERNEL_HEAP_SIZE: usize = 0x300000; // 3MB

pub const PAGE_SIZE: usize = 0x1000; // 4KB
pub const PAGE_SIZE_BITS: usize = 12;

pub const MEMORY_END: usize = 0x84000000; // 64MB

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

pub const CLOCK_FREQ: usize = 12500000; // 12.5MHz

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
