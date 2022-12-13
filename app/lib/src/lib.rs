// in Wii mode, enable `no_std` and import ogc_rs replacements of common functions.
#![cfg_attr(feature = "wii", no_std)]

#[cfg(feature = "wii")]
pub use ogc_rs::{print, println};

#[cfg(feature = "wii")]
extern crate alloc;

#[cfg(not(feature = "wii"))]
pub use std::{print, println};

pub mod game;


use hecs::{World, PreparedQuery};
use game::{Velocity, Position};

#[derive(Default)]
pub struct GameState {
    input_state: InputState,
    pub world: World,
    // velocity_query: PreparedQuery<&mut Velocity>,
    // all_query: PreparedQuery<(&mut Position, &mut Velocity)>,
}

/// Mapping of the Wii's input to a game-specific (and Wii-unspecific) format
#[derive(Default, Debug)]
pub struct InputState {
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl GameState {
    /// Construct a new GameState
    pub fn new() -> Self {
        let mut state: GameState = Default::default();
        game::batch_spawn_entities(&mut state.world, 5);
        state
    }

    /// Mutate the current GameState
    /// based on the new InputState
    /// Returns `false` as long as the game should continue running.
    /// If `true` is returned, the game loop should quit.
    ///
    /// Game loop logic lives in here
    pub fn update(&mut self, new_inputs: InputState) -> bool {
        if new_inputs.home_button_down {
            return true
        }

        if new_inputs.one_button_down {
            game::system_shake_wii(&mut self.world);
        }

        game::system_bounce_bounds(&mut self.world);

        self.input_state = new_inputs;

        return false;
    }
}


fn foo() -> bool {
    // println!("Hello, world!");
    println!("Hello, world!");
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_foo() {
        let result = super::foo();
        assert!(result);
    }
}
