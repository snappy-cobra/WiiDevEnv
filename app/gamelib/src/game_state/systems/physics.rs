use crate::game_state::GameState;
use crate::game_state::components::physics::SphereCollider;
use alloc::vec::Vec;
use crate::game_state::components::motion::{Position, Rotation};


/**
 * Main render system that renders meshinstance components.
 */
pub fn system_register_collider(state: &mut GameState) {
    let mut query = state.world.query::<&mut SphereCollider>();
    let mut colliders = query.iter().map(|(_e, c)| c).collect();
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    server_provider.render_server.register_collider(&mut colliders);
}

pub fn system_physics_to_position(state: &mut GameState) {
    let mut query = state.world.query::<(&mut SphereCollider, &mut Position, &mut Rotation)>();
    let mut colliders = query.iter().map(|(_e, c)| c).collect();
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    server_provider.render_server.physics_to_position(&mut colliders);
}
