use crate::game_state::GameState;

/**
 * Enumerates all game states that exist in the project.
 * Each of them can be turned into its actual state by calling `to_state()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameStateName {
    BouncingCubes,
}

impl GameStateName {
    /// Returns the state instance of this particular GameStateName
    pub const fn to_state(&self) -> fn(&mut GameState) {
        match self {
            GameStateName::BouncingCubes => system_exit_action,
        }
    }
}
