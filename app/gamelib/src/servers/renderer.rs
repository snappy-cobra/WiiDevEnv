use crate::game_state::GameState;

/**
 * Simple trait for implementing the wii specific renderer.
 */
pub trait RenderServer {
    fn render_state(&mut self, state: &GameState);
}