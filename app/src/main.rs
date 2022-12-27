#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]
#![feature(start)]
#![feature(slice_flatten)]
#![feature(slice_ptr_get)]

// Make sure the allocator is set.
extern crate alloc;
use ogc_rs::input::*;
use ogc_rs::prelude::*;

use hecs::*;

pub mod renderer;
use renderer::*;

mod game;
pub use game::*;

mod raw_data_store;

mod target_tests;

#[start]
/**
 * Main entrypoint of the application.
 */
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    if cfg!(feature = "run_target_tests") {
        println!("Running the target test suite...");
        target_tests::run_test_suite()
    } else {
        println!("Starting game!");
        main_game()
    }
}

fn main_game() -> isize {
    // Setup the wiimote
    Input::init(ControllerType::Wii);
    let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
    wii_mote
        .as_wpad()
        .set_data_format(WPadDataFormat::ButtonsAccelIR);

    // Setup the ECS environment.
    let mut world = World::new();
    batch_spawn_entities(&mut world, 5);
    let mut velocity_query = PreparedQuery::<&mut Velocity>::default();
    let mut all_query = PreparedQuery::<(&mut Position, &mut Velocity)>::default();

    // Kickstart main loop.
    let renderer = Renderer::new();
    loop {
        Input::update(ControllerType::Wii);
        if wii_mote.is_button_down(Button::Home) {
            break;
        }
        if wii_mote.is_button_down(Button::One) {
            system_shake_wii(&mut world, &mut velocity_query);
        }
        system_bounce_bounds(&mut world, &mut all_query);
        system_integrate_motion(&mut world, &mut all_query);

        renderer.render_world(&world);
    }
    0
}
