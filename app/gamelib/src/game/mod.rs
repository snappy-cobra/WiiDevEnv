use crate::game_state::{changes::ChangeProvider, GameState};
use crate::game_states::GameStateName;

/**
 * Main game loop struct that should handle the game flow, with help from
 * provided change provider and renderer. 
 */
pub struct Game {
    state: GameState,
    change_provider: ChangeProvider,
    renderer: Renderer
}

impl Game {
    /**
     * Construct a new game with the initial game state 
     * and wii specific change provider + renderer.
     */
    pub fn new(
        start_state: GameStateName, 
        change_provider: ChangeProvider, 
        renderer: Renderer
    ) -> Game {
        return Game {
            state: start_state.to_state(),
            change_provider,
            renderer
        };
    }

    /**
     * When called this method performs one update to the whole game,
     * which is one rendered frame.
     */
    pub fn update(&mut self) -> bool {
        // Get the changes and update the current state.
        let changes = self.change_provider.get_changes();
        let mut current_state = &mut self.state;
        let is_running = current_state.update(&changes);
        if ! is_running {
            return false;
        }

        // We are still running, so do the rest.
        self.renderer.render_state(& current_state);
        
        if current_state.next_state.exists() {
            // TODO : this is where you'd save the current scene, if you wanted that support.
            self.state = current_state.next_state.unwrap().to_state();
            // TODO: this is where you'd load a potential earlier save of the newly load scene, if you wanted that support.
        }

        return true;
    }
}

/**
 * Simple trait for implementing the wii specific renderer.
 */
pub trait Renderer {
    pub fn render_state(&self, state: &GameState);
}