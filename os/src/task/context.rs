//! Implementation of [`TaskContext`]

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TaskContext {
    /// return address (e.g. __restore_ctx) of __switch ASM function
    ra: usize,
    /// kernel stack pointer of app
    sp: usize,
    /// callee-saved registers
    s: [usize; 12],
}

impl TaskContext {
    pub fn new(ra: usize, sp: usize) -> Self {
        Self { ra, sp, s: [0; 12] }
    }

    /// empty init task context
    pub const fn empty() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}
