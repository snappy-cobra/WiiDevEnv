use crate::game_state::{changes::ChangeProvider, GameState};
use crate::game_states::{GameStateName, GameStateFactory};
use crate::servers::ServerProvider;

/**
 * Main game loop struct that should handle the game flow, with help from
 * provided change provider and renderer. 
 */
pub struct Game<C> {
    state: GameState,
    change_provider: C,
    server_provider: ServerProvider   
}

impl<C: ChangeProvider> Game<C> {
    /**
     * Construct a new game with the initial game state 
     * and wii specific change provider + renderer.
     */
    pub fn new(
        start_state: GameStateName, 
        change_provider: C, 
        server_provider: ServerProvider
    ) -> Self {
        return Self {
            state: GameStateFactory::to_state(start_state),
            change_provider,
            server_provider
        };
    }

    /**
     * When called this method performs one update to the whole game,
     * which is one rendered frame.
     */
    pub fn update(&mut self) -> bool {
        // Get the changes and update the current state.
        let changes = self.change_provider.get_changes();
        let is_running = self.state.update(changes);
        if ! is_running {
            return false;
        }

        // We are still running, so do the rest.
        self.server_provider.render_server.render_state(& self.state);

        match &self.state.next_state {
            Some(next_state) => {
                // TODO : this is where you'd save the current scene, if you wanted that support.
                self.state = GameStateFactory::to_state(next_state.clone())
                // TODO: this is where you'd load a potential earlier save of the newly load scene, if you wanted that support.
            },
            None => ()
        }

        return true;
    }
}