use crate::game_state::GameState;
use crate::game_state::components::motion::{Position, Rotation};
use crate::game_state::components::physics::SphereCollider;
use crate::game_state::components::render::MeshInstance;
use crate::game_state::components::game::Camera;
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

pub fn system_render_debug_physics(state: &mut GameState) {
    let mut query = state.world.query::<(&Position, &SphereCollider, &Rotation)>();
    let data = query.iter().map(|(_e, d)| d).collect();
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    server_provider.render_server.render_debug(data);
}

pub fn system_camera_update(state: &mut GameState) {
    for (id, (pos, camera)) in state.world.query_mut::<(&mut Position, &mut Camera)>() {
        let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
        server_provider.render_server.update_camera(pos, camera);
    }
}