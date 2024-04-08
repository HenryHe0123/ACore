use core::alloc::Layout;
use core::cmp::{max, min};
use core::fmt::Debug;
use core::mem::size_of;
use core::ptr::NonNull;

// buddy system with max order of 32
const MAX_ORDER: usize = 32;
// align size (what's the best?)
const ALIGN_SIZE: usize = size_of::<usize>();

type List = super::list::MyInList;

/// My alternative implementation for buddy_system_allocator::Heap
pub struct MyHeap {
    free_list: [List; MAX_ORDER],
    // statistics (for debug)
    allocated: usize, // memory allocated
    total: usize,     // total memory
}

// Required interfaces
impl MyHeap {
    /// Create an empty MyHeap
    pub const fn new() -> Self {
        Self {
            free_list: [List::new(); MAX_ORDER],
            allocated: 0,
            total: 0,
        }
    }

    pub fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    /// Alloc a range of memory from the heap satifying `layout` requirements
    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = aligned_layout_size(layout);
        let class = size.trailing_zeros() as usize;

        for i in class..MAX_ORDER {
            // find the first non-empty available size class
            if !self.free_list[i].is_empty() {
                // split it to the required size
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_list[j].pop() {
                        let half_len = (1 << (j - 1)) as usize;
                        let mid = (block as usize + half_len) as *mut usize;

                        self.free_list[j - 1].push(mid);
                        self.free_list[j - 1].push(block);
                    } else {
                        return Err(());
                    }
                }
                // split done, return the block
                let res = NonNull::new(self.free_list[class].pop().unwrap() as *mut u8);
                if let Some(res) = res {
                    self.allocated += size;
                    return Ok(res);
                } else {
                    return Err(());
                }
            }
        }
        Err(())
    }

    /// Dealloc a range of memory from the heap
    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = aligned_layout_size(layout);
        let class = size.trailing_zeros() as usize;

        // Put back into free list
        self.free_list[class].push(ptr.as_ptr() as *mut usize);

        // Merge free buddy lists
        let mut current_ptr = ptr.as_ptr() as usize;
        let mut current_class = class;
        while current_class < MAX_ORDER {
            let buddy = current_ptr ^ (1 << current_class);
            let current_list = &mut self.free_list[current_class];
            let flag = current_list.search_and_remove(buddy as *mut usize);

            // Free buddy found
            if flag {
                current_list.pop(); // Remove the current_ptr pushed before
                current_ptr = min(current_ptr, buddy);
                current_class += 1;
                self.free_list[current_class].push(current_ptr as *mut usize);
            } else {
                break;
            }
        }

        self.allocated -= size;
    }
}

// Some helper functions
impl MyHeap {
    /// Add memory [start, end) to the heap
    pub fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        // align start and end
        start = (start + ALIGN_SIZE - 1) & (!ALIGN_SIZE + 1);
        end = end & (!ALIGN_SIZE + 1);
        assert!(start <= end);

        // add at least one align size memory each time
        while start + ALIGN_SIZE <= end {
            // memory size to be added
            let size = min(start.lowbit(), (end - start).prev_power_of_two());

            self.free_list[size.trailing_zeros() as usize].push(start as *mut usize);
            self.total += size;
            start += size;
        }
    }
}

impl Debug for MyHeap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Heap")
            .field("allocated", &self.allocated)
            .field("total", &self.total)
            .finish()
    }
}

pub trait MyNum {
    /// Returns the largest power of two that divides `self`.
    ///
    /// e.g. lowbit(8) = 8, 6.lowbit(2) = 2.
    fn lowbit(self) -> usize;

    /// Returns the largest power of two less than or equal to `self`.
    fn prev_power_of_two(self) -> usize;
}

impl MyNum for usize {
    fn lowbit(self) -> usize {
        return self & (!self + 1);
    }

    fn prev_power_of_two(self) -> usize {
        return 1 << (8 * size_of::<usize>() - self.leading_zeros() as usize - 1);
    }
}

/// Returns the aligned size of the layout
fn aligned_layout_size(layout: Layout) -> usize {
    max(
        layout.size().next_power_of_two(),
        max(layout.align(), ALIGN_SIZE),
    )
}
