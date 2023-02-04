#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]
#![allow(clippy::all)]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;

use core::mem::MaybeUninit;
include!("physics.rs");

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Unit(pub f32);

impl Unit {
    /// float to int
    pub fn to_internal(&self) -> TPE_Unit {
        (self.0 * (TPE_F as f32)) as i32
    }
    /// int to float
    pub fn from_internal(val: TPE_Unit) -> Self {
        Self((val as f32) / (TPE_F as f32))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn to_internal(&self) -> TPE_Vec3 {
        TPE_Vec3 {
            x: Unit(self.0).to_internal(),
            y: Unit(self.1).to_internal(),
            z: Unit(self.2).to_internal(),
        }
    }

    pub fn from_internal(val: TPE_Vec3) -> Self {
        Self(
            Unit::from_internal(val.x).0,
            Unit::from_internal(val.y).0,
            Unit::from_internal(val.z).0,
        )
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Joint(TPE_Joint);

impl Joint {
    pub fn new(position: Vec3, size: f32) -> Self {
        let joint = unsafe { TPE_joint(position.to_internal(), Unit(size).to_internal()) };
        Self(joint)
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Connection(TPE_Connection);

impl Connection {
    pub fn new(joint1_index: u8, joint2_index: u8, length: u16) -> Self {
        Connection(TPE_Connection {
            joint1: joint1_index,
            joint2: joint2_index,
            length,
        })
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Body(TPE_Body);

impl Body {
    pub fn new(joints: &mut [Joint], connections: &mut [Connection], mass: f32) -> Self {
        let mut body = MaybeUninit::zeroed();
        let joints: &mut [TPE_Joint] = unsafe { core::mem::transmute(joints) };
        let joints_ptr = joints.as_mut_ptr();

        let connections: &mut [TPE_Connection] = unsafe { core::mem::transmute(connections) };
        let connections_ptr = connections.as_mut_ptr();

        unsafe {
            TPE_bodyInit(
                body.as_mut_ptr(),
                joints_ptr,
                joints.len().try_into().unwrap(),
                connections_ptr,
                connections.len().try_into().unwrap(),
                Unit(mass).to_internal(),
            )
        };
        let body = unsafe { body.assume_init() };
        Body(body)
    }

    pub fn apply_gravity(&mut self, downwards_acceleration: f32) {
        unsafe { TPE_bodyApplyGravity(&mut self.0, Unit(downwards_acceleration).to_internal()) };
    }

    /// Compute the center of mass for a body; average position of all joints.
    pub fn center_of_mass(&self) -> Vec3 {
        Vec3::from_internal(unsafe { TPE_bodyGetCenterOfMass(&self.0) })
    }

    // /// True if any forces are working on the body
    pub fn is_active(&self) -> bool {
        unsafe { TPE_bodyIsActive(&self.0) != 0 }
    }
}

#[derive(Debug, Clone)]
pub struct World(TPE_World);

impl World {
    pub fn new(bodies: &mut [Body]) -> Self {
        let bodies: &mut [TPE_Body] = unsafe { core::mem::transmute(bodies) };
        let bodies_ptr = bodies.as_mut_ptr();
        let mut world = MaybeUninit::zeroed();
        unsafe {
            TPE_worldInit(
                world.as_mut_ptr(),
                bodies_ptr,
                bodies.len().try_into().unwrap(),
                Some(infinitePlaneEnvDistance),
            )
        };
        let world = unsafe { world.assume_init() };
        World(world)
    }

    #[doc = " Performs one step (tick, frame, ...) of the physics world simulation"]
    #[doc = "including updating positions and velocities of bodies, collision detection and"]
    #[doc = "resolution, possible reshaping or deactivation of inactive bodies etc. The"]
    #[doc = "time length of the step is relative to all other units but it's ideal if it is"]
    #[doc = "1/30th of a second."]
    pub fn step(&mut self) {
        unsafe { TPE_worldStep(&mut self.0) };
    }
}

/// Taken from the simple example code
#[no_mangle]
pub extern "C" fn infinitePlaneEnvDistance(point: TPE_Vec3, _maxDistance: TPE_Unit) -> TPE_Vec3 {
    unsafe { TPE_envGround(point, 0) } // just an infinite flat plane
}

use typed_arena::Arena;

pub struct WorldWrapper {
    joints_arena: typed_arena::Arena<Vec<Joint>>,
    connections_arena: typed_arena::Arena<Vec<Connection>>,
    bodies_vec: Vec<Body>,
    world: World,
}

impl WorldWrapper {
    pub fn new() -> Self{
        Self {
            joints_arena: Arena::new(),
            connections_arena: Arena::new(),
            bodies_vec: vec![],
            world: World::new(&mut vec![]),
        }
    }

    pub fn add_body(&mut self, joints: Vec<Joint>, connections: Vec<Connection>, mass: f32) {
        let joints = self.joints_arena.alloc(joints);
        let connections = self.connections_arena.alloc(connections);
        let body = Body::new(joints, connections, mass);
        self.bodies_vec.push(body);
        self.fix_world_bodies_ptr();
    }

    fn fix_world_bodies_ptr(&mut self) {
        let ptr: *mut TPE_Body = unsafe { core::mem::transmute(self.bodies_vec.as_mut_ptr())};
        self.world.0.bodies = ptr;
        self.world.0.bodyCount = self.bodies_vec.len() as u16;
    }

    pub fn step(&mut self) {
        self.world.step();
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
}
