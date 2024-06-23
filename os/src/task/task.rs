use super::TaskContext;
use crate::config::TRAP_CONTEXT;
use crate::mm::address::*;
use crate::mm::memory_set::MemorySet;
use crate::mm::KERNEL_SPACE;
use crate::task::kernel_stack::KernelStack;
use crate::trap::*;
use crate::UPSafeCell;
use alloc::sync::Arc;
use core::cell::RefMut;

pub struct TaskControlBlock {
    // immutable
    pub pid: usize,
    pub kernel_stack: KernelStack,
    // mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

impl TaskControlBlock {
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        match self.try_borrow_mut() {
            Ok(task_inner) => {
                drop(task_inner);
            }
            Err(_) => {
                panic!("TCB inner is already borrowed for pid {}!", self.pid);
            }
        }
        self.inner.exclusive_access()
    }

    fn try_borrow_mut(
        &self,
    ) -> Result<RefMut<'_, TaskControlBlockInner>, core::cell::BorrowMutError> {
        self.inner.try_borrow_mut()
    }

    pub fn new(elf_data: &[u8], pre_alloc_pid: usize) -> Self {
        // 从elf文件中解析出内存布局
        let (memory_set, user_sp, entry_point) = MemorySet::new_from_elf(elf_data);
        // 获取trap_context的物理页号
        let trap_cx_ppn = memory_set
            .translate_to_ppn(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap();
        // 分配 pid 和 kernel stack
        let pid = pre_alloc_pid;
        let kernel_stack = KernelStack::new(pid);
        let kernel_stack_top = kernel_stack.get_top();
        // push a task context which goes to trap_return to the top of kernel stack
        let task_control_block = Self {
            pid,
            kernel_stack,
            inner: UPSafeCell::new(TaskControlBlockInner {
                trap_cx_ppn,
                task_cx: TaskContext::new(trap_return as usize, kernel_stack_top),
                memory_set,
            }),
        };
        // prepare Trap Context in user space
        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().satp_token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }
}

pub struct TaskControlBlockInner {
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.satp_token()
    }

    pub fn get_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.task_cx
    }
}

impl TaskControlBlock {
    pub fn fork(self: &Arc<TaskControlBlock>, new_pid: usize) -> Arc<TaskControlBlock> {
        // ---- access parent PCB exclusively
        let parent_inner = self.inner_exclusive_access();
        // copy user space (include trap context)
        let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        drop(parent_inner);
        let trap_cx_ppn = memory_set
            .translate_to_ppn(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap();

        // alloc kernel stack in kernel space
        let kernel_stack = KernelStack::new(new_pid);
        let kernel_stack_top = kernel_stack.get_top();
        let new_tcb = Arc::new(TaskControlBlock {
            pid: new_pid,
            kernel_stack,
            inner: UPSafeCell::new(TaskControlBlockInner {
                trap_cx_ppn,
                task_cx: TaskContext::new(trap_return as usize, kernel_stack_top),
                memory_set,
            }),
        });
        // modify kernel_sp in new trap_cx
        // ---- access children PCB exclusively
        let trap_cx = new_tcb.inner_exclusive_access().get_trap_cx();
        trap_cx.kernel_sp = kernel_stack_top;
        // return
        new_tcb
    }

    pub fn exec(&self, elf_data: &[u8]) {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, user_sp, entry_point) = MemorySet::new_from_elf(elf_data);
        let trap_cx_ppn = memory_set
            .translate_to_ppn(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap();
        // ---- access inner exclusively
        let mut inner = self.inner_exclusive_access();
        // substitute memory_set
        inner.memory_set = memory_set;
        // update trap_cx ppn
        inner.trap_cx_ppn = trap_cx_ppn;
        // initialize trap_cx
        let trap_cx = inner.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().satp_token(),
            self.kernel_stack.get_top(),
            trap_handler as usize,
        );
        // ---- stop exclusively accessing inner automatically
    }
}
