use alloc::vec;
use alloc::vec::Vec;
use crate::game_state::components::controller_assignment::FryAssignment;
use crate::game_state::components::motion::{Position, Rotation};
use crate::game_state::components::physics::SphereCollider;
use crate::game_state::GameState;


pub fn system_score_frying_pans(state: &mut GameState) {
    let mut potato_body_indexes = Vec::new();
    for (_id, potato) in state.world.query_mut::<(&mut SphereCollider)>() {
        potato_body_indexes.push(potato.body_index);
    }
    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
    for (_id, (position, fry_assignment)) in state.world.query_mut::<(&mut Position, &mut FryAssignment)>() {
        server_provider.render_server.fry_pan_score_increase(position, fry_assignment, &potato_body_indexes);
    }
}