// in Wii mode, enable `no_std` and import ogc_rs replacements of common functions.
#![cfg_attr(feature = "wii", no_std)]
#[cfg(feature = "wii")]
pub use ogc_rs::{print, println};
#[cfg(feature = "wii")]
extern crate alloc;

#[cfg(not(feature = "wii"))]
pub use std::{print, println};

use hecs::World;
use num::clamp;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use core::time::Duration;

#[allow(dead_code)]
fn foo() -> bool {
    println!("Hello, world!");
    true
}

/// Represents the state of the game
///
/// This includes all entities, as well as potentially score, 'current mode' and other global data
pub struct GameState {
    pub world: World,
}

/// Represents any changes made by the outside world during a single frame.
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Changes {
    pub controls: Controls,
    pub delta_time: Duration,
}

/// Represents the state of one or multiple Wii controllers w.r.t. the game
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Controls {
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl GameState {
    /// Creates a GameState in the situation at the start of the game
    pub fn new() -> GameState {
        let mut res = GameState {
            world: World::new(),
        };

        batch_spawn_entities(&mut res.world, 5);

        res
    }

    /// Performs game logic of one iteration of the game loop.
    /// Before calling this, the caller should have filled `Changes` with the required values.
    ///
    /// If the returned bool is `true`, the game loop should continue.
    /// When `false` is returned, the game should quit.
    pub fn update(&mut self, changes: &Changes) -> bool {
        if changes.controls.home_button_down {
            return false;
        }

        if changes.controls.one_button_down {
            system_shake_wii(&mut self.world)
        }

        system_bounce_bounds(&mut self.world);
        system_integrate_motion(&mut self.world);

        true
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
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
        world.spawn((position, velocity));
    }
}

/**
 * Apply the velocity to the positions.
 */
pub fn system_integrate_motion(world: &mut World) {
    const DRAG: f32 = 1.001;
    for (_id, (position, velocity)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
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
pub fn system_bounce_bounds(world: &mut World) {
    for (_id, (position, velocity)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
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

/**
 * Create velocity in random directions.
 */
pub fn system_shake_wii(world: &mut World) {
    let mut small_rng = SmallRng::seed_from_u64(10u64);
    for (_id, velocity) in world.query_mut::<&mut Velocity>() {
        velocity.x += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
        velocity.y += small_rng.next_u32() as f32 / u32::MAX as f32 * 0.2 - 0.1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 3;
        assert_eq!(result, 5);
    }

    #[test]
    fn test_foo() {
        let result = super::foo();
        assert!(result);
    }
}
