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
struct Unit(f32);

impl Unit {
    /// float to int
    pub fn to_internal(&self) -> TPE_Unit {
        self.0 * TPE_F
    }
    /// int to float
    pub fn from_internal(val: TPE_Unit) -> Self {
        Self(val / TPE_F)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn to_internal(&self) -> TPE_Vec3 {
        TPE_Vec3 {
            x: self.0.to_internal(),
            y: self.1.to_internal(),
            z: self.2.to_internal(),
        }
    }

    pub fn from_internal(val: TPE_Vec3) -> Self {
        Self(
            Unit::from_internal(val.x),
            Unit::from_internal(val.y),
            Unit::from_internal(val.z),
        )
    }
}

#[derive(Debug, Clone)]
pub struct Joint(TPE_Joint);

impl Joint {
    pub fn new(position: Vec3, size: Unit) -> Self {
        let joint = unsafe { TPE_joint(position.to_internal(), size.to_internal()) };
        Self(joint)
    }
}

#[derive(Debug, Clone)]
pub struct Connection(TPE_Connection);

impl Connection {
    pub fn new(joint1_index: u8, joint2_index: u8, length: u16) -> Self {
        Connection(TPE_Connection { joint1: joint_a_index, joint2: joint_b_index, length })
    }
}

#[derive(Debug, Clone)]
pub struct Body(TPE_Body);

impl Body {
    pub fn new(joints: &[Joint], connections: &[Connection], mass: Unit) -> Self {
        let mut body = MaybeUninit::zeroed();
        unsafe { TPE_bodyInit(body.as_mut_ptr(), joints, joints.len(), connections, connections.len(), mass) };
        let body = unsafe { body.assume_init() };
        Body(body)
    }

    pub fn apply_gravity(&mut self, downwards_acceleration: Unit) {
        unsafe { TPE_bodyApplyGravity(self.0, downwards_acceleration.to_internal()) };
    }

    /// Compute the center of mass for a body; average position of all joints.
    pub fn center_of_mass(&self) -> Vec3 {
        Vec3::from_internal(unsafe { TPE_bodyGetCenterOfMass(self.0) })
    }

    /// True if any forces are working on the body
    pub fn is_active(&self) -> bool {
        unsafe { TPE_bodyIsActive(self.0) }
    }
}

#[derive(Debug, Clone)]
pub struct World(TPE_World);

impl World {
    pub fn new(bodies: &[Bodies]) -> Self {
        let mut world = MaybeUninit::zeroed();
        unsafe { TPE_worldInit(world.as_mut_ptr(), bodies, bodies.len(), &infinitePlaneEnvDistance)};
        let world = unsafe { world.assume_init() };
        World(world)
    }

    #[doc = " Performs one step (tick, frame, ...) of the physics world simulation"]
    #[doc = "including updating positions and velocities of bodies, collision detection and"]
    #[doc = "resolution, possible reshaping or deactivation of inactive bodies etc. The"]
    #[doc = "time length of the step is relative to all other units but it's ideal if it is"]
    #[doc = "1/30th of a second."]
    pub fn step(&mut self) {
        unsafe { TPE_worldStep(self.0) };
    }
}

/// Taken from the simple example code
fn infinitePlaneEnvDistance(point: TPE_Vec3, maxDistance: TPE_Unit) -> TPE_Vec3
{
    unsafe { TPE_envGround(point,0) } // just an infinite flat plane
}

#[cfg(test)]
pub mod tests{
    use libc::putchar;

    use super::*;

    #[test]
    pub fn example() {
        let joint = Joint::new(Vec3(0.0, 8.0, 0.0), 1.0);
        let mut body = Body::new(&[joint], &[], 2.0);
        let mut world = World::new(&[body]);

        let frame: usize = 0;
        while body.is_active() {
            if (frame % 6 == 0) {
                let height = body.center_of_mass().1;
                for _index in 0..(height * 4) {
                    print!(' ');
                }
                print!("*");
            }

            body.apply_gravity(1.0 / 100);
            world.step();
        }
        println!("body deactivated");
        assert!(false);
    }
}
