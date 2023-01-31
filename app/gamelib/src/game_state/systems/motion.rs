use crate::game_state::GameState;
use crate::game_state::components::audio::Audio;
use crate::game_state::components::motion::{Position, Velocity};
use crate::servers::audio::PlayMode;
use crate::data_store::asset_name::AssetName;
use num::clamp;

/**
 * Apply the velocity to the positions.
 */
pub fn system_integrate_motion(state: &mut GameState) {
    const DRAG: f32 = 1.001;
    for (_id, (position, velocity)) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        position.x += velocity.x;
        velocity.x /= DRAG;
        position.y += velocity.y;
        velocity.y /= DRAG;
        position.z += velocity.z;
        velocity.z /= DRAG;
    }
}

/**
 * Bounce the cubes against the bounds.
 */
pub fn system_bounce_bounds(state: &mut GameState) {
    let mut play_boing = false;
    
    for (_id, (position, velocity)) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        const BOX_SIZE: f32 = 5.0;
        if position.x > BOX_SIZE {
            velocity.x *= -1.0;
            position.x = BOX_SIZE;
            play_boing = true;
        } else if position.x < -1.0 * BOX_SIZE {
            velocity.x *= -1.0;
            position.x = -BOX_SIZE;
            play_boing = true;
        }
        if position.y > BOX_SIZE {
            velocity.y *= -1.0;
            position.y = BOX_SIZE;
            play_boing = true;
        } else if position.y < -1.0 * BOX_SIZE {
            velocity.y *= -1.0;
            position.y = -BOX_SIZE;
            play_boing = true;
        }
        if position.z > BOX_SIZE {
            velocity.z *= -1.0;
            position.z = BOX_SIZE;
            play_boing = true;
        } else if position.z < -1.0 * BOX_SIZE {
            velocity.z *= -1.0;
            position.z = -BOX_SIZE;
            play_boing = true;
        }
        velocity.x = clamp(velocity.x, -0.5, 0.5);
        velocity.y = clamp(velocity.y, -0.5, 0.5);
        velocity.z = clamp(velocity.z, -0.5, 0.5);
    }

    if play_boing {
        let boing = Audio::new(AssetName::BoingSFX, PlayMode::OneTime);
        state.world.spawn((boing,));
    }
}
