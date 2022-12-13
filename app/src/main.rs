
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
use ogc_rs::input::Input;
use ogc_rs::input::controller::{ControllerType, Button};

use hecs::*;

pub mod renderer;
use renderer::*;

mod game;
pub use game::*;

use rust_wii_lib::{GameState, InputState};

#[start]
/**
 * Main entrypoint of the application.
 */
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    println!("Hello Rust!");

    let wii_mote = setup_controllers();

    let mut state = GameState::new();

    // Kickstart main loop.
    let mut renderer = Renderer::new();
    renderer.init_render();
    loop {
        let new_input_state = update_controllers(& wii_mote);
        let should_quit = state.update(new_input_state);
        if should_quit {
            break;
        }

        renderer.render_world(&state.world);
    }
    renderer.close_render();
    return 0;
}

fn setup_controllers() -> Input {
    // Setup the wiimote
    Input::init(ControllerType::Wii);
    let wii_mote1 = Input::new(ControllerType::Wii, ControllerPort::One);
    wii_mote1
        .as_wpad()
        .set_data_format(WPadDataFormat::ButtonsAccelIR);

    wii_mote1
}

fn update_controllers(wii_mote: &Input) -> InputState {
    Input::update(ControllerType::Wii);

    InputState {
        home_button_down: wii_mote.is_button_down(Button::Home),
        one_button_down: wii_mote.is_button_down(Button::One)
    }
}
