
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]
#![feature(start)]

// Make sure the allocator is set.
extern crate alloc;
use ogc_rs::prelude::*;
use ogc_rs::input::*;

use hecs::*;

pub mod renderer;
use renderer::*;

mod game;
pub use game::*;

#[start]
/**
 * Main entrypoint of the application.
 */
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    WPad::init();
    let wii_mote = WPad::new(ControllerPort::One);
    let mut world = World::new();
    batch_spawn_entities(&mut world, 5);
    //let mut motion_query = PreparedQuery::<(&mut Vector3, &Vector3)>::default();
    
    init_render();
    loop {
        WPad::update();
        if wii_mote.is_button_down(WPadButton::HOME) {
            break
        }
        //system_integrate_motion(&mut world, &mut motion_query);
        render_world(&world);
    }
    close_render();
    return 0;
}