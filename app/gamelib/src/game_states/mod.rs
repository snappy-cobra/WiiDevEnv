use crate::game_state::GameState;

mod bouncing_cubes;

/**
 * Enumerates all game states that exist in the project.
 * Each of them can be turned into its actual state by calling `to_state()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum GameStateName {
    BouncingCubes,
}

/**
 * Returns the state instance of the given particular GameStateName
 */
pub struct GameStateFactory {}
impl GameStateFactory {
    pub fn to_state(name: GameStateName) -> GameState {
        match name {
            GameStateName::BouncingCubes => bouncing_cubes::build(),
        }
    }
}