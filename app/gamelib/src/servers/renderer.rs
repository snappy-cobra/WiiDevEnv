use crate::game_state::components::{render::MeshInstance, motion::Position, motion::Rotation, physics::SphereCollider};
#[cfg(feature = "wii")]
use ogc_rs::prelude::Vec;
#[cfg(not(feature = "wii"))]
use std::vec::Vec;
use crate::game_state::changes::controls::Direction;

/**
 * Simple trait for implementing the wii specific renderer.
 */
pub trait RenderServer {
    fn render_meshes(&mut self, meshes: Vec::<(&MeshInstance, &Position, &Rotation)>);
    fn render_debug(&mut self, meshes: Vec::<(&Position, &SphereCollider, &Rotation)>);
    fn render_frame(&mut self);
    fn register_collider(&mut self, colliders: &mut Vec::<&mut SphereCollider>);
    fn world_step(&mut self);
    fn physics_to_position(&mut self, temp: &mut Vec<(&mut SphereCollider, &mut Position, &mut Rotation)>);
    fn apply_movement(&mut self, obj: &SphereCollider, dir: Direction);
}
