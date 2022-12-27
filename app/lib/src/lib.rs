// in Wii mode, enable `no_std` and import ogc_rs replacements of common functions.
#![cfg_attr(feature = "wii", no_std)]
#[cfg(feature = "wii")]
pub use ogc_rs::{print, println};
#[cfg(feature = "wii")]
extern crate alloc;

#[cfg(not(feature = "wii"))]
pub use std::{print, println};


#[allow(dead_code)]
fn foo() -> bool {
    println!("Hello, world!");
    true
}

/// Represents the state of the game
///
/// This includes all entities, as well as potentially score, 'current mode' and other global data
pub struct GameState {
    world: hecs::World,
}

/// Represents any changes made by the outside world during a single frame.
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Changes {
    pub controls: Controls,
    pub delta_time_ms: u64,
}

/// Represents the state of one or multiple Wii controllers w.r.t. the game
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Controls {
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl GameState {
    /// Creates a GameState in the situation at the start of the game
    pub fn new() -> GameState {
        GameState { world: hecs::World::new() }
    }

    /// Performs game logic of one iteration of the game loop.
    /// Before calling this, the caller should have filled `Changes` with the required values.
    ///
    /// If the returned bool is `true`, the game loop should continue.
    /// When `false` is returned, the game should quit.
    pub fn update(&mut self, changes: &Changes) {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 3;
        assert_eq!(result, 5);
    }

    #[test]
    fn test_foo() {
        let result = super::foo();
        assert!(result);
    }
}
