use super::address::*;
use super::frame_allocator::*;
use super::page_table::*;
use crate::config::PAGE_SIZE;
use alloc::collections::BTreeMap;
use bitflags::bitflags;

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

/// VM area (RAII)
pub struct MapArea {
    pub vpn_range: VPNRange,
    map_type: MapType,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            map_type,
            data_frames: BTreeMap::new(),
            map_perm,
        }
    }

    pub fn new_by_varange(va_range: VARange, map_type: MapType, map_perm: MapPermission) -> Self {
        let vpn_range = VPNRange::new(va_range.start.floor(), va_range.end.ceil());
        Self {
            vpn_range,
            map_type,
            data_frames: BTreeMap::new(),
            map_perm,
        }
    }

    pub fn map_to(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map(page_table, vpn);
        }
    }

    pub fn unmap_to(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap(page_table, vpn);
        }
    }

    /// data: start-aligned but maybe with shorter length
    ///
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.start;
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table
                .translate_to_ppn(current_vpn)
                .unwrap()
                .get_bytes_array()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
}

impl MapArea {
    /// Map one virtual page to page table.
    pub fn map(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    /// Unmap one virtual page from page table.
    pub fn unmap(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        match self.map_type {
            MapType::Framed => {
                self.data_frames.remove(&vpn);
            }
            _ => {}
        }
        page_table.unmap(vpn);
    }
}
