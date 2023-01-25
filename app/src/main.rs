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
use core::time::Duration;
use libc::exit;
use ogc_rs::clock::Instant;
use ogc_rs::input::*;
use ogc_rs::prelude::*;

use gamelib::{Changes, Controls, GameState};
use grrustlib::{STM_ShutdownToStandby, SYS_SetPowerCallback};
use hecs::*;

use modulator::sources::*;
use modulator::*;

pub mod rendering;
use rendering::renderer::*;

pub mod input;
use input::InputManager;

mod raw_data_store;

mod audio;
mod target_tests;
use audio::ogg_player::{OGGPlayer, PlayMode};
use raw_data_store::AssetName;

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
    register_power_callback();
    let ogg_player = OGGPlayer::new(Asnd::init());
    let mut game_state = GameState::new();
    let mut input_manager = InputManager::new();
    let mut renderer = Renderer::new();

    let mut modenv: ModulatorEnv<f32> = Default::default();
    modenv.take("myfancywave", Box::new(Wave::new(2.0, 0.5))); // start with 2.0 amplitude and 0.5Hz frequency)
    let mut now = Instant::now();

    ogg_player.set_volume(100);
    ogg_player.play(&AssetName::DemoMusic, PlayMode::Infinite);

    while KEEP_RUNNING.load(Ordering::SeqCst) {
        let (delta_time, new_now) = calculate_delta_time(&now);
        now = new_now;

        let controls = input_manager.update();

        let changes = Changes {
            controls,
            delta_time,
        };
        let should_continue = game_state.update(&changes);
        if !should_continue {
            break;
        }
        modenv.advance(
            delta_time
                .as_nanos()
                .try_into()
                .expect("Overflow in duration"),
        );

        renderer.render_world(&game_state.world);
    }
    //ogg_player.stop();
    shutdown()
}

/// Given the Instant returned the last time this function was called,
/// will calculate the duration elapsed since then, and returns a new Instant
/// to be passed back to this function next iteration.
pub fn calculate_delta_time(earlier: &Instant) -> (Duration, Instant) {
    let now = Instant::now();
    let delta_time = elapsed_between(earlier, &now);
    (delta_time, now)
}

/// Calculate the `Duration` between two different `Instant`s
pub fn elapsed_between(start: &Instant, end: &Instant) -> Duration {
    Duration::from_nanos(Instant::from_ticks(end.ticks - start.ticks).nanosecs())
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
