use crate::game_state::GameState;
use crate::game_state::components::physics::SphereCollider;
use alloc::vec::Vec;


/**
 * Main render system that renders meshinstance components.
 */
pub fn system_register_collider(state: &mut GameState) {
    let mut query = state.world.query::<& SphereCollider>();
    let colliders: Vec<& SphereCollider> = query.iter().map(|(_e, (c))| (c)).collect();
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    server_provider.render_server.register_collider(colliders);
}