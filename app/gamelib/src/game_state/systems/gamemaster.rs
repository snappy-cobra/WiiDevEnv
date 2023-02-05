use crate::game_state::components::render::MeshInstance;
use crate::game_state::*;
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
    system_animation(state);
    system_camera_movement(state);
    system_game_start(state);
    system_game_finish(state);
}

pub fn system_game_start(state: &mut GameState) {
    match state.playmode {
        PlayMode::Playing | PlayMode::Hands | PlayMode::Finish => {}
        PlayMode::Selection => {
            let tmp_position = Position{
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let tmp_animation = Animation {
                duration: 0.0,
                past_time: 0.0,
                animation_type: AnimationType::None,
                on_animation_finish: OnAnimationFinish::Hand3,
                target_x: 0.0,
                target_y: 0.0,
                target_z: 0.0,
            };
    
            state.world.spawn((tmp_position, tmp_animation));
            state.playmode = PlayMode::Hands; // TODO: BUTTON click
        }
    }
}

pub fn system_game_finish(state: &mut GameState) {
    
}

pub fn system_camera_movement(state: &mut GameState) {
    for (_id, (_pos, _camera)) in state.world.query_mut::<(&mut Position, &mut Camera)>() {
        // pos.y -= state.changes.delta_time.as_secs_f32();
    }
}

pub fn lerp(a: f32, b: f32, t:f32) -> f32 {
    return a + (b-a) * t; 
}

pub fn system_animation(state: &mut GameState) {
    let mut to_remove: Vec<Entity> = Vec::new();
    let mut to_add: Vec<(Position, Rotation, Animation, MeshInstance)> = Vec::new();
    let mut startPlaying: bool = false;
    

    for (id, (pos, animation)) in state.world.query_mut::<(&mut Position, &mut Animation)>() {
        animation.past_time += state.changes.delta_time.as_secs_f32();

        match animation.animation_type {
            AnimationType::None => {}
            AnimationType::Test => {
                pos.x += state.changes.delta_time.as_secs_f32();
                pos.y += state.changes.delta_time.as_secs_f32();
                pos.z += state.changes.delta_time.as_secs_f32();
            }
            AnimationType::Bubble => {
                pos.y += state.changes.delta_time.as_secs_f32();
            }
            AnimationType::HandIn | AnimationType::HandOut => {
                let mut t = animation.past_time / animation.duration;
                pos.x = lerp(pos.x, animation.target_x, t);
                pos.y = lerp(pos.y, animation.target_y, t);
                pos.z = lerp(pos.z, animation.target_z, t);
            }
            AnimationType::Hand => {
                pos.y += state.changes.delta_time.as_secs_f32();
            }
        }
        
        if animation.past_time >= animation.duration {
            match  animation.on_animation_finish {
                OnAnimationFinish::Repeat => { 
                    animation.past_time -= animation.duration; 
                    pos.x = animation.target_x; 
                    pos.y = animation.target_y; 
                    pos.z = animation.target_z; 
                }
                OnAnimationFinish::RepeatBubble => { 
                    animation.past_time -= animation.duration; 
                    let tmp = pos.x;
                    pos.x = pos.z; 
                    pos.y = animation.target_y; 
                    pos.z = tmp; 
                }
                OnAnimationFinish::Despawn => { to_remove.push(id) }

                OnAnimationFinish::Hand3 => {
                    let hand_mesh = MeshInstance { model_name: TexturedModelName::HandThree };
                    let hand_position = Position{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.5,
                        past_time: 0.0,
                        animation_type: AnimationType::HandIn,
                        on_animation_finish: OnAnimationFinish::Hand2,
                        target_x: 0.0,
                        target_y: 10.0,
                        target_z: 10.0,
                    };
                    let hand_rotation = Rotation { x: 0.0, y: 90.0, z: 0.0 };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh)); 
                    to_remove.push(id); 
                } 

                OnAnimationFinish::Hand2 => { 
                    print!("switch to hand 2");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand1,
                        target_x: pos.x, target_y: pos.y, target_z: pos.z, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandTwo };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand1 => { 
                    print!("switch to hand 1");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand0,
                        target_x: pos.x, target_y: pos.y, target_z: pos.z, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandOne };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand0 => { 
                    print!("switch to hand 0");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 0.5,
                        past_time: 0.0,
                        animation_type: AnimationType::HandOut,
                        on_animation_finish: OnAnimationFinish::Start,
                        target_x: 0.0, target_y: pos.y -10.0, target_z: 40.0, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandFist };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Start => {
                    print!("switch to Start");
                    startPlaying = true;
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

    if startPlaying {
        state.playmode = PlayMode::Playing;
    }
}