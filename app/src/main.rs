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
use core::sync::atomic::{AtomicBool, Ordering};
use ogc_rs::input::*;
use ogc_rs::prelude::*;

use hecs::*;

pub mod renderer;
use renderer::*;

mod game;
use crate::plot::{Plot, PlotsHolder};
pub use game::*;

mod raw_data_store;

mod controller;
mod plot;
mod target_tests;

/// Global flag to signal to the main game loop when the game should quit.
///
/// Starts of 'true', and is toggled whenever someone physically tries to power down the Wii/shut off the game.
///
/// C.f. `register_power_callback`
static KEEP_RUNNING: AtomicBool = AtomicBool::new(true);

/**
 * Main entrypoint of the application.
 */

#[start]
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
    println!("Hello Rust!");
    register_power_callback();

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
    // let mut gforce_plot_vec: Vec<Vec<f32>> = Vec::new();
    let mut plotHolder = PlotsHolder::new();
    while KEEP_RUNNING.load(Ordering::SeqCst) {
        Input::update(ControllerType::Wii);
        if wii_mote.is_button_down(Button::Home) {
            break;
        }
        if wii_mote.is_button_down(Button::One) {
            system_shake_wii(&mut world, &mut velocity_query);
        }
        controller::plot_wiimote_movement(&mut plotHolder, &wii_mote);
        system_bounce_bounds(&mut world, &mut all_query);
        system_integrate_motion(&mut world, &mut all_query);

        renderer.render_world(&world);
    }
    shutdown();
}

/// Registers the power callback,
/// ensuring that when someone tries to shutdown the Wii by pressing the power button,
/// we first can do cleanup and shut down cleanly afterwards
///
/// Without this, the Wii would hang when the user would try to exit the game.
///
pub fn register_power_callback() {
    unsafe { SYS_SetPowerCallback(Some(power_callback)) };
}

/// Callback to be registered as power callback
///
/// Toggles a global flag which is checked inside the game loop, to break from it.
extern "C" fn power_callback() {
    println!("Received a shutdown call");
    KEEP_RUNNING.store(false, Ordering::SeqCst);
}

/// Instructs the system to shut down cleanly.
pub fn shutdown() -> ! {
    unsafe {
        STM_ShutdownToStandby();
    }
    core::unreachable!()
}
