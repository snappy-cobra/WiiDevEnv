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

mod audio;
mod change_provider;
mod input;
mod rendering;
mod target_tests;

use audio::ogg_server::WiiOGGServer;
use change_provider::WiiChangeProvider;
use core::sync::atomic::{AtomicBool, Ordering};
use gamelib::{
    data_store::asset_name::AssetName,
    game::Game,
    game_states::GameStateName,
    servers::{audio::AudioServer, renderer::RenderServer, ServerProvider},
};
use grrustlib::{STM_ShutdownToStandby, SYS_SetPowerCallback};
use input::InputManager;
use libc::exit;
use ogc_rs::prelude::*;
use rendering::render_server::WiiRenderServer;

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

    let audio_server = WiiOGGServer::new(Asnd::init());
    let render_server = WiiRenderServer::new();
    let server_provider = ServerProvider::new(audio_server, render_server);

    let mut game = Game::new(
        GameStateName::BouncingCubes,
        WiiChangeProvider::new(InputManager::new()),
        server_provider,
    );

    while KEEP_RUNNING.load(Ordering::SeqCst) {
        let should_continue = game.update();
        if !should_continue {
            break;
        }
    }
    shutdown()
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
