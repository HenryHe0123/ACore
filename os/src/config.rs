pub const USER_STACK_SIZE: usize = 4096 * 2; // 8KB
pub const KERNEL_STACK_SIZE: usize = 4096 * 2; // 8KB
pub const KERNEL_HEAP_SIZE: usize = 0x300000; // 3MB

pub const PAGE_SIZE: usize = 0x1000; // 4KB
pub const PAGE_SIZE_BITS: usize = 12;

pub const MEMORY_END: usize = 0x80800000; // 8MB
