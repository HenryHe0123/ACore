#![no_std]
#![no_main]

use user_lib::read_from_shared_page;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() {
    let flag = read_from_shared_page(0);
    println!("Read from shared page: {}", flag);
}
