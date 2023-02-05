use crate::game_state::components::game::Camera;
use crate::game_state::components::{render::MeshInstance, motion::Position, motion::Rotation, physics::SphereCollider};
#[cfg(feature = "wii")]
use ogc_rs::prelude::Vec;
use ogc_rs::println;
#[cfg(not(feature = "wii"))]
use std::vec::Vec;
use crate::game_state::changes::controls::Direction;
use crate::game_state::components::controller_assignment::FryAssignment;
use crate::game_state::GameState;
use crate::game_states::GameStateName;

/**
 * Simple trait for implementing the wii specific renderer.
 */
pub trait RenderServer {
    fn render_meshes(&mut self, meshes: Vec::<(&MeshInstance, &Position, &Rotation)>);
    fn render_debug(&mut self, meshes: Vec::<(&Position, &SphereCollider, &Rotation)>);
    fn render_frame(&mut self);
    fn update_camera(&mut self, pos: &Position, cam: &Camera);
    fn register_collider(&mut self, colliders: &mut Vec::<&mut SphereCollider>);
    fn world_step(&mut self);
    fn physics_to_position(&mut self, temp: &mut Vec<(&mut SphereCollider, &mut Position, &mut Rotation)>);
    fn teleport_potato(&mut self, temp: &mut Vec<(&mut SphereCollider, &mut Position, &mut Rotation)>);
    fn apply_movement(&mut self, obj: &SphereCollider, dir: Direction);
    fn reset_world(&mut self);
    fn fry_pan_score_increase(&mut self, position: &mut Position, fry_assignment: &mut FryAssignment, potatoes: &Vec<usize>);
}
