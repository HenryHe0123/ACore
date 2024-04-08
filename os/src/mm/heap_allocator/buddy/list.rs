use crate::debug;
use core::ptr;

/// A naive intrusive linked list
///
/// only the pointer is stored in the node
#[derive(Copy, Clone)]
pub struct MyInList {
    head: *mut usize,
}

impl MyInList {
    /// Create a new MyInList
    pub const fn new() -> Self {
        Self {
            head: ptr::null_mut(),
        }
    }

    /// Return `true` if the list is empty
    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    /// Push `item` to the front of the list
    pub fn push(&mut self, item: *mut usize) {
        unsafe {
            *item = self.head as usize;
        }
        self.head = item;
    }

    /// Try to remove the first item in the list
    pub fn pop(&mut self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => {
                let item = self.head;
                self.head = unsafe { *item as *mut usize };
                Some(item)
            }
        }
    }
}

impl MyInList {
    /// Search and remove `item` from the list, return `false` if not found
    ///
    /// No guarantee of efficiency
    pub fn search_and_remove(&mut self, item: *mut usize) -> bool {
        type Ptr = *mut usize;
        let mut prev = &mut self.head as *mut Ptr;
        let mut curr = self.head;
        while !curr.is_null() {
            unsafe {
                if curr == item {
                    *prev = *curr as Ptr;
                    return true;
                }
                prev = curr as *mut Ptr;
                curr = *curr as Ptr;
            }
        }
        false
    }
}

#[allow(unused)]
pub fn list_test() {
    let mut value1: usize = 0;
    let mut value2: usize = 0;
    let mut value3: usize = 0;
    let mut list = MyInList::new();

    list.push(&mut value1 as *mut usize);
    list.push(&mut value2 as *mut usize);
    list.push(&mut value3 as *mut usize);

    // Test links
    assert_eq!(value3, &value2 as *const usize as usize);
    assert_eq!(value2, &value1 as *const usize as usize);
    assert_eq!(value1, 0);

    // Test pop
    assert_eq!(list.pop(), Some(&mut value3 as *mut usize));
    assert_eq!(list.pop(), Some(&mut value2 as *mut usize));
    assert_eq!(list.pop(), Some(&mut value1 as *mut usize));
    assert_eq!(list.pop(), None);

    debug!("[test] list test passed!");
}
