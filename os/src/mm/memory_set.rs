use super::address::VirtAddr;
use super::map_area::*;
use super::page_table::PageTable;
use alloc::vec::Vec;

/// Address Space (RAII)
pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
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
}
