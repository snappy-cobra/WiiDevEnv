
#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::prelude::*;
use grrlib::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        GRRLIB_Init();
        PAD_Init();
        GRRLIB_Exit();
    }

    return 0;
}