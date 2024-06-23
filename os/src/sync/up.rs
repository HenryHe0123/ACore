//! unit processor safe call

use core::cell::{RefCell, RefMut};

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    /// User is responsible to guarantee that inner struct is only used in uniprocessor.
    pub const fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    /// Panic if the data has been borrowed.
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        // self.inner.borrow_mut()
        match self.inner.try_borrow_mut() {
            Ok(borrow) => borrow,
            Err(_) => {
                panic!(
                    "BorrowMutError: The type of T is {}",
                    core::any::type_name::<T>()
                );
            }
        }
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, core::cell::BorrowMutError> {
        self.inner.try_borrow_mut()
    }
}
