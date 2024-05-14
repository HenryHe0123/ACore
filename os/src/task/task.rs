use super::TaskContext;
use crate::config::kernel_stack_position;
use crate::config::TRAP_CONTEXT;
use crate::mm::address::*;
use crate::mm::map_area::MapPermission;
use crate::mm::memory_set::MemorySet;
use crate::mm::KERNEL_SPACE;
use crate::trap::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub memory_set: MemorySet,
    pub trap_cx_ppn: PhysPageNum,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        // 从elf文件中解析出内存布局
        let (memory_set, user_sp, entry_point) = MemorySet::new_from_elf(elf_data);
        // 获取trap_context的物理页号
        let trap_cx_ppn = memory_set
            .translate_to_ppn(VirtAddr::from(TRAP_CONTEXT).into())
            .unwrap();
        // 在内核地址空间创建kernel stack映射
        let (kernel_stack_bottom, kernel_stack_top) = kernel_stack_position(app_id);
        KERNEL_SPACE.exclusive_access().insert_empty_framed_area(
            kernel_stack_bottom.into(),
            kernel_stack_top.into(),
            MapPermission::R | MapPermission::W,
        );
        // init task control block
        let task_control_block = Self {
            task_status: TaskStatus::Ready,
            task_cx: TaskContext::new(trap_return as usize, kernel_stack_top),
            memory_set,
            trap_cx_ppn,
        };
        // prepare TrapContext in user space
        let trap_cx = task_control_block.get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().satp_token(),
            kernel_stack_top,
            trap_handler as usize,
        );
        task_control_block
    }

    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    pub fn get_user_token(&self) -> usize {
        self.memory_set.satp_token()
    }
}
