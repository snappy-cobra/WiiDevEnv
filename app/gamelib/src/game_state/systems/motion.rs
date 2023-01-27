use crate::game_state::GameState;
use crate::game_state::components::motion::{Position, Velocity};
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
    for (_id, (position, velocity)) in state.world.query_mut::<(&mut Position, &mut Velocity)>() {
        const BOX_SIZE: f32 = 5.0;
        if position.x > BOX_SIZE {
            velocity.x *= -1.0;
            position.x = BOX_SIZE;
        } else if position.x < -1.0 * BOX_SIZE {
            velocity.x *= -1.0;
            position.x = -BOX_SIZE;
        }
        if position.y > BOX_SIZE {
            velocity.y *= -1.0;
            position.y = BOX_SIZE;
        } else if position.y < -1.0 * BOX_SIZE {
            velocity.y *= -1.0;
            position.y = -BOX_SIZE;
        }
        if position.z > BOX_SIZE {
            velocity.z *= -1.0;
            position.z = BOX_SIZE;
        } else if position.z < -1.0 * BOX_SIZE {
            velocity.z *= -1.0;
            position.z = -BOX_SIZE;
        }
        velocity.x = clamp(velocity.x, -0.5, 0.5);
        velocity.y = clamp(velocity.y, -0.5, 0.5);
        velocity.z = clamp(velocity.z, -0.5, 0.5);
    }
}