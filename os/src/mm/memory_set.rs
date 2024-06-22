use super::address::*;
use super::map_area::*;
use super::page_table::*;
use crate::asm;
use crate::config::*;
use crate::sbi::mmio::MMIO;
use alloc::vec::Vec;
use riscv::register::satp;

/// Address Space (RAII)
pub struct MemorySet {
    pub page_table: PageTable,
    pub areas: Vec<MapArea>,
}

impl MemorySet {
    pub fn from_existed_user(user_space: &MemorySet) -> MemorySet {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // copy data sections/trap_context/user_stack
        for area in user_space.areas.iter() {
            let new_area = MapArea::from_another(area);
            memory_set.push(new_area, None);
            // copy data from another space
            for vpn in area.vpn_range {
                let src_ppn = user_space.translate_to_ppn(vpn).unwrap();
                let dst_ppn = memory_set.translate_to_ppn(vpn).unwrap();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(src_ppn.get_bytes_array());
            }
        }
        memory_set
    }

    pub fn recycle_data_pages(&mut self) {
        self.areas.clear();
    }
}

impl MemorySet {
    pub fn translate_to_ppn(&self, vpn: VirtPageNum) -> Option<PhysPageNum> {
        self.page_table.translate_to_ppn(vpn)
    }

    pub fn satp_token(&self) -> usize {
        self.page_table.satp_token()
    }
}

impl MemorySet {
    /// Remove `MapArea` that starts with `start_vpn`
    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.start == start_vpn)
        {
            area.unmap_to(&mut self.page_table);
            self.areas.remove(idx);
        }
    }
}

// --------------------------- MemorySet construct methods --------------------------------

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
    pub fn insert_empty_framed_area(
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

    fn map_shared_page(&mut self) {
        self.page_table.map(
            VirtAddr::from(SHARED_PAGE).into(),
            PhysAddr::from(SHARED_PAGE).into(),
            PTEFlags::R | PTEFlags::W | PTEFlags::U,
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

// user space

impl MemorySet {
    /// Include sections in elf and trampoline and TrapContext and user stack,
    /// also returns user_sp and entry point.
    pub fn new_from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // map shared page
        memory_set.map_shared_page();
        // debug!("new_from_elf0");
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        // debug!("new_from_elf2");
        let elf_header = elf.header;
        // check magic number
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        // get program header count
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        // traverse program headers
        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
                max_end_vpn = map_area.vpn_range.end;
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                );
            }
        }
        // map user stack with U flags
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_bottom: usize = max_end_va.into();
        // guard page
        user_stack_bottom += PAGE_SIZE;
        let user_stack_top = user_stack_bottom + USER_STACK_SIZE;
        // map user stack
        memory_set.insert_empty_framed_area(
            user_stack_bottom.into(),
            user_stack_top.into(),
            MapPermission::R | MapPermission::W | MapPermission::U,
        );
        // map TrapContext
        memory_set.insert_empty_framed_area(
            (TRAMPOLINE - PAGE_SIZE).into(),
            TRAMPOLINE.into(),
            MapPermission::R | MapPermission::W,
        );
        // return user_space, user_sp, entry_point
        (
            memory_set,
            user_stack_top,
            elf.header.pt2.entry_point() as usize,
        )
    }
}
