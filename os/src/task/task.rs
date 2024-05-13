use super::TaskContext;
use crate::mm::address::PhysPageNum;
use crate::mm::memory_set::MemorySet;

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
    pub base_size: usize,
}

impl TaskControlBlock {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        unimplemented!()
    }
}
