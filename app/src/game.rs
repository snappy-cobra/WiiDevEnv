use hecs::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::RngCore;
use ogc_rs::{print, println, prelude::Input};
use num::clamp;

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/**
 * Spawn multiple entities in the world
 */
pub fn batch_spawn_entities(world: &mut World, n: i32) {
    let mut small_rng = SmallRng::seed_from_u64(10u64);
    for index in 0..n {
        const row_width : i32 = 10;
        let pos_x : f32 = (index % row_width) as f32;
        let pos_z : f32 = (index / row_width) as f32;

        let position = Position {
            x: pos_x,
            y: 0.0,
            z: pos_z,
        };
        let velocity = Velocity {
            x: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
            y: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
            z: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1
        };
        world.spawn((position, velocity));
    };
}

/**
 * Apply the velocity to the positions.
 */
pub fn system_integrate_motion(
    world: &mut World, 
    query: &mut PreparedQuery<(&mut Position, &mut Velocity)>
) {
    const drag : f32 = 1.001;
    for (_id, (position, velocity)) in query.query_mut(world) {
        position.x += velocity.x;
        velocity.x = velocity.x / drag;
        position.y += velocity.y;
        velocity.y = velocity.y / drag;
        position.z += velocity.z;
        velocity.z = velocity.z / drag;
    }
}

/**
 * Bounce the cubes against the bounds.
 */
pub fn system_bounce_bounds(
    world: &mut World, 
    query: &mut PreparedQuery<(&mut Position, &mut Velocity)>
) {
    for (_id, (position, velocity)) in query.query_mut(world) {
        const box_size : f32 = 5.0;
        if position.x > box_size {
            velocity.x = -1.0 * velocity.x;
            position.x = box_size;
        } else if position.x < -1.0*box_size {
            velocity.x = -1.0 * velocity.x;
            position.x = -box_size;
        }
        if position.y > box_size {
            velocity.y = -1.0 * velocity.y;
            position.y = box_size;
        } else if position.y < -1.0*box_size {
            velocity.y = -1.0 * velocity.y;
            position.y = -box_size;
        }
        if position.z > box_size {
            velocity.z = -1.0 * velocity.z;
            position.z = box_size;
        } else if position.z < -1.0*box_size {
            velocity.z = -1.0 * velocity.z;
            position.z = -box_size;
        }
        velocity.x = clamp(velocity.x, -0.5, 0.5);
        velocity.y = clamp(velocity.y, -0.5, 0.5);
        velocity.z = clamp(velocity.z, -0.5, 0.5);
    }
}

/**
 * Create velocity in random directions.
 */
pub fn system_shake_wii(
    world: &mut World, 
    query: &mut PreparedQuery<&mut Velocity>
) {
    let mut small_rng = SmallRng::seed_from_u64(10u64);
    for (_id, velocity) in query.query_mut(world) {
        velocity.x += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
        velocity.y += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
    }
}
