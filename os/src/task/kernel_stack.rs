use crate::config::kernel_stack_position;
use crate::mm::address::VirtAddr;
use crate::mm::map_area::MapPermission;
use crate::mm::KERNEL_SPACE;
use crate::task::pid::Pid;

/// Kernel stack for app
pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    /// Create a kernel stack from pid
    pub fn new(pid: &Pid) -> Self {
        let pid = pid.0;
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(pid);
        KERNEL_SPACE.exclusive_access().insert_empty_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        KernelStack { pid }
    }

    #[allow(unused)]
    /// Push a value on top of kernel stack
    pub fn push_on_top<T>(&self, value: T) -> *mut T
    where
        T: Sized,
    {
        let top = self.get_top();
        let ptr_mut = (top - core::mem::size_of::<T>()) as *mut T;
        unsafe {
            *ptr_mut = value;
        }
        ptr_mut
    }

    /// Get the value on the top of kernel stack
    pub fn get_top(&self) -> usize {
        let (_, kernel_stack_top) = kernel_stack_position(self.pid);
        kernel_stack_top
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (kernel_stack_bottom, _) = kernel_stack_position(self.pid);
        let kernel_stack_bottom_va: VirtAddr = kernel_stack_bottom.into();
        KERNEL_SPACE
            .exclusive_access()
            .remove_area_with_start_vpn(kernel_stack_bottom_va.into());
    }
}
