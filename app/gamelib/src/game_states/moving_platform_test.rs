use hecs::World;
use num::ToPrimitive;

use crate::data_store::asset_name::AssetName;
use crate::game_state::GameState;
use crate::game_state::components::audio::Audio;
use crate::game_state::components::game::*;
use crate::game_state::components::motion::*;
use crate::game_state::components::render::MeshInstance;
use crate::game_state::systems::system_name::SystemName;
use crate::servers::audio::PlayMode;
use crate::data_store::textured_model_name::TexturedModelName;
use alloc::vec::Vec;
use alloc::vec;

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

/**
 * Build the bouncing cubes game state.
 */
pub fn build() -> GameState {
    let mut state = GameState::new();
    state.add_system(SystemName::RenderMeshes);
    state.add_system(SystemName::MovingPlatform);
    state.add_system(SystemName::GameMaster);
    batch_spawn_entities(&mut state.world, 10);
    return state;
}

/**
 * Spawn multiple entities in the world
 */
fn batch_spawn_entities(world: &mut World, n: i32) {
    let mut small_rng = SmallRng::seed_from_u64(10u64);

    let position = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let rotation = Rotation {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mesh_instance = MeshInstance { model_name: TexturedModelName::Plate };
    world.spawn((position, rotation, mesh_instance));


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
        animation_type: AnimationType::Test,
        on_animation_finish: OnAnimationFinish::Hand2,
        target_x: 0.0,
        target_y: 0.0,
        target_z: 0.0,
    };
    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandThree };
    world.spawn((hand_position, hand_rotation, hand_animation, hand_mesh_instance));


}