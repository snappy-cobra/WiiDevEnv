#![feature(const_mut_refs)]
// In Wii mode, enable `no_std` and import ogc_rs replacements of common functions.
#![cfg_attr(feature = "wii", no_std)]
#[cfg(feature = "wii")]
pub use ogc_rs::{print, println};
#[cfg(feature = "wii")]
extern crate alloc;
#[cfg(not(feature = "wii"))]
pub use std::{print, println};

pub mod game_state;
pub mod game_states;
pub mod game;
pub mod plot;
pub mod data_store;
pub mod servers;
