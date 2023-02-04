use crate::game_state::{GameState, components::game::Platform};
use crate::game_state::components::motion::*;
use crate::game_state::components::game::*;

/**
 * Move the Platform.
 */
pub fn system_moving_platform(state: &mut GameState) {
    
}

pub fn system_gamemaster(state: &mut GameState) {
    system_animation(state)
}

pub fn system_animation(state: &mut GameState) {
    // let mut to_despawn: Vec::new();

    for (id, (pos, animation)) in state.world.query_mut::<(&mut Position, &mut Animation)>() {
        animation.past_time += state.changes.delta_time.as_secs_f32();

        match animation.animation_type {
            AnimationType::Test => {
                pos.x += state.changes.delta_time.as_secs_f32();
                pos.y += state.changes.delta_time.as_secs_f32();
                pos.z += state.changes.delta_time.as_secs_f32();
            }
        }
        
        if (animation.past_time >= animation.duration) {
            match  animation.on_animation_finish {
                OnAnimationFinish::Despawn => { pos.x = 99999.0; } // TODO: despawn
            }
        }
    }

    // for id in to_despawn.iter() {
    //     state.world.despawn(id);
    // }
}