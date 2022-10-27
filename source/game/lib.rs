#![no_std]
#![allow(dead_code)]

use core::{panic::PanicInfo};

extern "C" { pub fn c_function(); }

#[no_mangle]
pub extern "C" fn rust_function() {
    unsafe {c_function();}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}