#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// FIXME: This allow will eventually cancel an error, eventually remove it.
#![allow(unaligned_references)]
#![allow(unused_imports)]
#![no_std]

#![allow(clippy::all)]

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
    pub fn new(joints: &[Joint], connections: &[Connection], mass: f32) -> Self {
        let mut body = MaybeUninit::zeroed();
        let joints: &[TPE_Joint] = unsafe { core::mem::transmute(joints) };
        let joints_ptr = joints.as_ptr();

        let connections: &[TPE_Connection] = unsafe { core::mem::transmute(connections) };
        let connections_ptr = connections.as_ptr();

        unsafe {
            TPE_bodyInit(
                body.as_mut_ptr(),
                joints_ptr as *mut TPE_Joint,
                joints.len().try_into().unwrap(),
                connections_ptr as *mut TPE_Connection,
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
    pub fn new(bodies: &[Body]) -> Self {
        let bodies: &[TPE_Body] = unsafe { core::mem::transmute(bodies) };
        let bodies_ptr = bodies.as_ptr();
        let mut world = MaybeUninit::zeroed();
        unsafe {
            TPE_worldInit(
                world.as_mut_ptr(),
                bodies_ptr as *mut TPE_Body,
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

#[cfg(test)]
pub mod tests {

    use super::*;
}
