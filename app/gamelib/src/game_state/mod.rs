
use hecs::World;
use ogc_rs::prelude::Vec;
use core::time::Duration;

mod systems;
mod system_name;

/// Represents the state of the game
///
/// This includes all entities, as well as potentially score, 'current mode' and other global data
pub struct GameState {
    pub world: World,
    pub changes: Changes,
    systems: Vec<fn(&mut GameState)>,
    is_running: bool
}

/**
 * Implementation of the game state.
 */
impl GameState {
    pub fn from_json_string(json_string: &str) -> Result<GameState, &'static str> {
        let mut res = GameState {
            world: World::new(),
            changes: Changes::nothing(),
            systems: Vec::new(),
            is_running: true
        };

        // TODO: parse json description.

        res
    }

    /**
     * Add a new system to the game state.
     */
    pub fn add_system(&mut self, system: fn(&mut GameState)) {
        self.systems.push(system);
    }

    /// Performs game logic of one iteration of the game loop.
    /// Before calling this, the caller should have filled `Changes` with the required values.
    ///
    /// If the returned bool is `true`, the game loop should continue.
    /// When `false` is returned, the game should quit.
    pub fn update(&mut self, changes: &Changes) -> bool {
        self.changes = changes;
        for system in self.systems {
            system(&self);
        }
        return self.is_running
    }
}

/// Represents any changes made by the outside world during a single frame.
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Changes {
    pub controls: Controls,
    pub delta_time: Duration,
}

impl Changes {
    pub fn nothing() -> Changes {
        return Changes {
            controls: Controls::nothing(),
            delta_time: Duration::new(0, 0)
        };
    }
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

impl Controls {
    pub fn nothing() -> Controls {
        return Controls {
            home_button_down: false,
            one_button_down: false,
        }
    }
}