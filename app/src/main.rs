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
use rust_wii_lib::{Changes, Controls, GameState};

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
    let mut game_state = GameState::new();
    let mut input_manager = InputManager::new();
    let renderer = Renderer::new();

    loop {
        let controls = input_manager.update();

        let changes = Changes {
            controls,
            delta_time_ms: 100,
        };
        let should_continue = game_state.update(&changes);
        if !should_continue {
            break;
        }

        renderer.render_world(&game_state.world);
    }
    0
}

struct InputManager {
    wii_mote: Input,
}

impl InputManager {
    pub fn new() -> Self {
        // Setup the wiimote
        Input::init(ControllerType::Wii);
        let wii_mote = Input::new(ControllerType::Wii, ControllerPort::One);
        wii_mote
            .as_wpad()
            .set_data_format(WPadDataFormat::ButtonsAccelIR);
        Self { wii_mote }
    }

    pub fn update(&mut self) -> Controls {
        Input::update(ControllerType::Wii);
        Controls {
            home_button_down: self.wii_mote.is_button_down(Button::Home),
            one_button_down: self.wii_mote.is_button_down(Button::One),
        }
    }
}

impl Default for InputManager {
    fn default() -> Self {
        Self::new()
    }
}
