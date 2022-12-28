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
use libc::exit;
use ogc_rs::input::*;
use ogc_rs::prelude::*;

use hecs::*;
use rust_wii_lib::{Changes, Controls, GameState};

pub mod renderer;
use renderer::*;

mod raw_data_store;

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
        register_power_callback();
        main_game();
        shutdown();
        0
    }
}

fn main_game() -> isize {
    // register_power_callback();
    let mut game_state = GameState::new();
    let mut input_manager = InputManager::new();
    let renderer = Renderer::new();

    while KEEP_RUNNING.load(Ordering::SeqCst) {
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
    println!("Test me");
    // shutdown();
    0
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
    // unsafe {
    //     STM_ShutdownToStandby();
    // }
    unsafe { exit(0) };
    // core::unreachable!()
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
