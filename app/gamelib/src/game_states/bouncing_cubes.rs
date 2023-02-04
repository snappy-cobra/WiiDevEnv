use hecs::World;

use crate::data_store::asset_name::AssetName;
use crate::game_state::GameState;
use crate::game_state::components::audio::Audio;
use crate::game_state::components::motion::{Position, Velocity};
use crate::game_state::components::render::MeshInstance;
use crate::game_state::systems::system_name::SystemName;
use crate::servers::audio::PlayMode;
use crate::data_store::textured_model_name::TexturedModelName;

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;

/**
 * Build the bouncing cubes game state.
 */
pub fn build() -> GameState {
    let mut state = GameState::new();
    state.add_system(SystemName::PlayAudio);
    state.add_system(SystemName::ExitAction);
    state.add_system(SystemName::StopAction);
    state.add_system(SystemName::ShakeAction);
    state.add_system(SystemName::IntegrateMotion);
    state.add_system(SystemName::BounceBounds);
    state.add_system(SystemName::RenderMeshes);
    batch_spawn_entities(&mut state.world, 10);
    spawn_main_music(&mut state.world);
    return state;
}

/**
 * The main music is an entity with an audio component.
 */
fn spawn_main_music(world: &mut World) {
    let audio = Audio::new(AssetName::DemoMusic, PlayMode::Infinite);
    world.spawn((audio,));
}

/**
 * Spawn multiple entities in the world
 */
fn batch_spawn_entities(world: &mut World, n: i32) {
    let mut small_rng = SmallRng::seed_from_u64(10u64);
    for index in 0..n {
        const ROW_WIDTH: i32 = 10;
        let pos_x: f32 = (index % ROW_WIDTH) as f32;
        let pos_z: f32 = (index / ROW_WIDTH) as f32;

        let position = Position {
            x: pos_x,
            y: 0.0,
            z: pos_z,
        };
        let velocity = Velocity {
            x: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
            y: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
            z: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
        };
        let mesh_instance = MeshInstance { model_name: TexturedModelName::Potato };
        world.spawn((mesh_instance, position, velocity));
    }
}