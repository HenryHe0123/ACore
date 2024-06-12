//! Loading user applications into memory

use crate::info;
use alloc::vec::Vec;
use lazy_static::lazy_static;

extern "C" {
    fn _num_app();
}

/// Get the total number of applications.
pub fn get_num_app() -> usize {
    unsafe { (_num_app as usize as *const usize).read_volatile() }
}

/// in ELF format
pub fn get_app_data(app_id: usize) -> &'static [u8] {
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    assert!(app_id < num_app);
    unsafe {
        core::slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}

// load by name ------------------------------------

lazy_static! {
    /// store all app names
    static ref APP_NAMES: Vec<&'static str> = {
        let num_app = get_num_app();
        extern "C" {
            fn _app_names();
        }
        let mut start = _app_names as usize as *const u8;
        let mut v = Vec::new();
        unsafe {
            for _ in 0..num_app {
                let mut end = start;
                while end.read_volatile() != b'\0' {
                    end = end.add(1);
                }
                let slice = core::slice::from_raw_parts(start, end as usize - start as usize);
                let str = core::str::from_utf8(slice).unwrap();
                v.push(str);
                start = end.add(1);
            }
        }
        v
    };
}

#[allow(unused)]
pub fn get_app_data_by_name(name: &str) -> Option<&'static [u8]> {
    (0..get_num_app())
        .find(|&i| APP_NAMES[i] == name)
        .map(get_app_data)
}

pub fn list_apps() {
    info!("[kernel] ----- APPS -----");
    for app in APP_NAMES.iter() {
        info!("[kernel] {}", app);
    }
    info!("[kernel] ----------------")
}
