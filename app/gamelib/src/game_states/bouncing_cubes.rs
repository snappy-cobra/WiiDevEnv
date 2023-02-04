use hecs::World;
use num::ToPrimitive;

use crate::data_store::asset_name::AssetName;
use crate::game_state::GameState;
use crate::game_state::components::audio::Audio;
use crate::game_state::components::motion::{Position, Velocity, Rotation};
use crate::game_state::components::render::MeshInstance;
use crate::game_state::components::game::*;
use crate::game_state::systems::system_name::SystemName;
use crate::servers::audio::PlayMode;
use crate::data_store::textured_model_name::TexturedModelName;

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use crate::game_state::components::physics::SphereCollider;
use crate::game_state::components::controller_assignment::ControllerAssignment;

/**
 * Build the bouncing cubes game state.
 */
pub fn build() -> GameState {
    let mut state = GameState::new();
    state.add_system(SystemName::GameMaster);
    state.add_system(SystemName::PlayAudio);
    state.add_system(SystemName::ExitAction);
    state.add_system(SystemName::RegisterCollider);
    state.add_system(SystemName::StopAction);
    // state.add_system(SystemName::ShakeAction);
    // state.add_system(SystemName::IntegrateMotion);
    // state.add_system(SystemName::BounceBounds);
    state.add_system(SystemName::PatatoControl);
    state.add_system(SystemName::PhysicsToPosition);
    state.add_system(SystemName::RenderMeshes);
    state.add_system(SystemName::CameraUpdate);
    state.add_system(SystemName::ResetLevel);

    batch_spawn_entities(&mut state.world, 8);
    spawn_main_music(&mut state.world);
    
    //state.add_system(SystemName::DebugPhysics);
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
    let cam_position = Position {
        x: 0.0,
        y: 35.0,
        z: 10.0,
    };
    let camera = Camera {
        r: 0x00,        g: 0x00,        b: 0x00,
        up_x: 0.0,      up_y: 1.0,      up_z: 0.0,
        lookat_x: 0.0,  lookat_y: 0.0,  lookat_z: 0.0,
    };
    world.spawn((camera, cam_position));


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
        let rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        
        let mesh_instance = MeshInstance { model_name: TexturedModelName::Potato };
        let sphere_collider = SphereCollider{radius: 1.0, gravity: true, body_index: 0, has_been_registered: false};
        let controller_assignment = ControllerAssignment{
            id: 0,
        };
        world.spawn((mesh_instance, position, velocity, rotation, sphere_collider, controller_assignment));
    }

    let plate_mesh = MeshInstance { model_name: TexturedModelName::Plate };
    let plate_position = Position{
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let plate_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
    world.spawn((plate_mesh, plate_position, plate_rotation));


    let fry_mesh = MeshInstance { model_name: TexturedModelName::FryPan };
    let fry_position = Position{
        x: 0.0,
        y: -10.0,
        z: 0.0,
    };
    let fry_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
    world.spawn((fry_mesh, fry_position, fry_rotation));


    let oil_mesh = MeshInstance { model_name: TexturedModelName::OilSea };
    let oil_position = Position{
        x: 0.0,
        y: -10.0,
        z: 0.0,
    };
    let oil_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
    world.spawn((oil_mesh, oil_position, oil_rotation));


    for index in 0..10 {
        let bubble_mesh = MeshInstance { model_name: TexturedModelName::OilBubble };
        let bubble_position = Position{
            x: small_rng.next_u32() as f32 % 20.0 as f32 -10.0,
            y: 0.0,
            z: small_rng.next_u32() as f32 % 20.0 as f32 -10.0,
        };
        let bubble_animation = Animation {
            duration: 3.0,
            past_time: small_rng.next_u32() as f32 % 3.0 as f32,
            animation_type: AnimationType::Bubble,
            on_animation_finish: OnAnimationFinish::Repeat,
        };
        let bubble_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        world.spawn((bubble_mesh, bubble_position, bubble_animation, bubble_rotation));
    }
}
