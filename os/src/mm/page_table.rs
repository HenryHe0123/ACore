use super::address::*;
use super::frame_allocator::*;
use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;

bitflags! {
    /// page table entry flags
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }

    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    // some auxiliary methods
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    #[allow(dead_code)]
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    #[allow(dead_code)]
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    #[allow(dead_code)]
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<FrameTracker>,
    // further bind the lifecycle of frames to the page table
    // may not in order
}

impl PageTable {
    pub fn new() -> Self {
        // for root
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: vec![frame],
        }
    }

    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        if pte.is_valid() {
            panic!("vpn {:?} is mapped before mapping", vpn);
        }
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        if !pte.is_valid() {
            panic!("vpn {:?} is not mapped before unmapping", vpn);
        }
        *pte = PageTableEntry::empty();
    }
}

impl PageTable {
    /// find pte with create
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for i in 0..2 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            // if invalid, create a new pte node
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        let pte = &mut ppn.get_pte_array()[idxs[2]];
        Some(pte)
    }

    /// find pte without create
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        for i in 0..2 {
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        let pte = &mut ppn.get_pte_array()[idxs[2]];
        Some(pte)
    }
}

impl PageTable {
    /// Build satp token for self.
    pub fn satp_token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }

    /// Translate vpn to pte.
    pub fn translate_to_pte(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| pte.clone())
    }

    /// Translate vpn to ppn.
    pub fn translate_to_ppn(&self, vpn: VirtPageNum) -> Option<PhysPageNum> {
        self.find_pte(vpn).map(|pte| pte.ppn())
    }
}
