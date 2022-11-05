
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![no_std]
#![feature(start)]

extern crate alloc;
use ogc_rs::prelude::*;

include!("grrlib.rs");

mod inline;
pub use inline::*;

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    unsafe {
        GRRLIB_Init();
        PAD_Init();
        GRRLIB_Exit();
    }

    return 0;
}