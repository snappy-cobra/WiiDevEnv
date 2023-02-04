use crate::game_state::GameState;
use crate::game_state::components::motion::{Position, Rotation};
use crate::game_state::components::render::MeshInstance;
#[cfg(feature = "wii")]
use ogc_rs::prelude::Vec;
#[cfg(not(feature = "wii"))]
use std::vec::Vec;

/**
 * Main render system that renders meshinstance components.
 */
pub fn system_render_meshes(state: &mut GameState) {
    let mut query = state.world.query::<(&MeshInstance, &Position, &Rotation)>();
    let mesh_instances: Vec<(&MeshInstance, &Position, &Rotation)> = query.iter().map(|(_e, (m, p, r))| (m, p, r)).collect();
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    server_provider.render_server.render_meshes(mesh_instances);
}
