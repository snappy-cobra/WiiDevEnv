mod systems;
pub mod changes;
mod components;
pub mod game_state_name;

use hecs::World;
use ogc_rs::prelude::Vec;
use changes::Changes;
use systems::system_name::SystemName;
use game_state_name::GameStateName;

/// Represents the state of the game
///
/// This includes all entities, as well as potentially score, 'current mode' and other global data
pub struct GameState {
    pub world: World,
    pub changes: Changes,
    pub next_state: Option<GameStateName>,
    pub is_running: bool,
    systems: Vec<SystemName>,
}

/**
 * Implementation of the game state.
 */
impl GameState {
    pub fn new() -> GameState {
        GameState {
            world: World::new(),
            changes: Changes::nothing(),
            systems: Vec::new(),
            is_running: true,
            next_state: None,
        }
    }

    /**
     * Add a new system to the game state.
     */
    pub fn add_system(&mut self, system: SystemName) {
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
            system.to_function()(&mut self);
        }
        return self.is_running
    }
}