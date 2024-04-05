use super::address::*;
use super::map_area::*;
use super::page_table::*;
use crate::asm;
use crate::config::MEMORY_END;
use crate::config::TRAMPOLINE;
use crate::sbi::mmio::MMIO;
use alloc::vec::Vec;
use riscv::register::satp;

/// Address Space (RAII)
pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
}

impl MemorySet {
    /// Create a new address space.
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
        }
    }

    /// Push a new map area into the address space.
    fn push(&mut self, mut area: MapArea, data: Option<&[u8]>) {
        area.map_to(&mut self.page_table);
        // write initial data (optional)
        if let Some(data) = data {
            area.copy_data(&mut self.page_table, data);
        }
        self.areas.push(area);
    }

    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(start_va, end_va, MapType::Framed, permission),
            None,
        );
    }

    /// Enable address space.
    pub fn activate(&self) {
        let satp = self.page_table.satp_token();
        unsafe {
            satp::write(satp);
            asm!("sfence.vma"); // flush TLB
        }
        // debug!("memory set activated");
    }

    /// Mention that trampoline is not collected by areas.
    fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }
}

// kernel space

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

// in blue
macro_rules! print_kernel_init_info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!(
            "\x1b[34m", "[kernel] ", $fmt, "\x1b[0m", "\n"
        ) $(, $($arg)+)?));
    }
}

impl MemorySet {
    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // map kernel sections
        print_kernel_init_info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
        print_kernel_init_info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
        print_kernel_init_info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
        print_kernel_init_info!(
            ".bss [{:#x}, {:#x})",
            sbss_with_stack as usize,
            ebss as usize
        );
        print_kernel_init_info!("mapping .text section");
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        print_kernel_init_info!("mapping .rodata section");
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
        );
        print_kernel_init_info!("mapping .data section");
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        print_kernel_init_info!("mapping .bss section");
        memory_set.push(
            MapArea::new(
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        print_kernel_init_info!("mapping physical memory");
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // debug: Don't forget to map MMIO!
        print_kernel_init_info!("mapping MMIO");
        for va_range in MMIO {
            let ma = MapArea::new_by_varange(
                va_range,
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            );
            memory_set.push(ma, None);
        }
        memory_set
    }
}
