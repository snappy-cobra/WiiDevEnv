
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


/**
 * Main entrypoint of the application.
 */

static mut running: bool = false;
#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello Rust!");
    unsafe {
        running = true
    }
    let power_callback_function_pointer: unsafe extern "C" fn() = power_callback;
    let s = Some(power_callback_function_pointer);
    unsafe { SYS_SetPowerCallback(s) };
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

    let mut renderer = Renderer::new();
    renderer.init_render();
    loop {
        // Check the loop should keep on running
        unsafe {
            if !running { break; }
        }
        Input::update(ControllerType::Wii);
        if wii_mote.is_button_down(Button::Home) {
            // break;
            unsafe { STM_ShutdownToStandby(); }
        }
        if wii_mote.is_button_down(Button::One) {
            system_shake_wii(&mut world, &mut velocity_query);
        }
        system_bounce_bounds(&mut world, &mut all_query);
        system_integrate_motion(&mut world, &mut all_query);

        renderer.render_world(&world);
    }
    renderer.close_render();
    return 0;
}

pub unsafe extern "C" fn power_callback() {
    println!("Received a shutdown call");
    unsafe {
        running = false
    }
    // Here STM_ShutdownToStandby() does not work
}