use crate::game_state::components::render::MeshInstance;
use crate::game_state::{GameState, components::game::Platform};
use crate::game_state::components::motion::*;
use crate::game_state::components::game::*;
use crate::data_store::textured_model_name::TexturedModelName;
use alloc::vec::Vec;
use alloc::vec;
use hecs::{DynamicBundle, Entity, Component};
use ogc_rs::print;

/**
 * Move the Platform.
 */
pub fn system_moving_platform(state: &mut GameState) {
    
}

pub fn system_gamemaster(state: &mut GameState) {
    system_animation(state)
}


pub fn system_animation(state: &mut GameState) {
    let mut to_remove: Vec<Entity> = Vec::new();
    let mut to_add: Vec<(Position, Rotation, Animation, MeshInstance)> = Vec::new();
    

    for (id, (pos, animation)) in state.world.query_mut::<(&mut Position, &mut Animation)>() {
        animation.past_time += state.changes.delta_time.as_secs_f32();

        match animation.animation_type {
            AnimationType::None => {}
            AnimationType::Test => {
                pos.x += state.changes.delta_time.as_secs_f32();
                pos.y += state.changes.delta_time.as_secs_f32();
                pos.z += state.changes.delta_time.as_secs_f32();
            }
        }
        
        if (animation.past_time >= animation.duration) {
            match  animation.on_animation_finish {
                OnAnimationFinish::Repeat => { animation.past_time -= animation.duration; }
                OnAnimationFinish::Despawn => { to_remove.push(id) }

                OnAnimationFinish::Hand2 => { 
                    print!("switch to hand 2");
                    let hand_position = Position {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0,
                        y: 90.0,
                        z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 2.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand1,
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandTwo };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand1 => { 
                    print!("switch to hand 1");
                    let hand_position = Position {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0,
                        y: 90.0,
                        z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 2.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand0,
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandOne };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand0 => { 
                    print!("switch to hand 0");
                    let hand_position = Position {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0,
                        y: 90.0,
                        z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 2.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Start,
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandFist };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Start => {
                    print!("switch to Start");
                    to_remove.push(id); 
                }
            }
        }
    }

    for id in to_remove.into_iter() {
        state.world.despawn(id);
    }
    for comps in to_add.into_iter() {
        state.world.spawn(comps);
    }
}