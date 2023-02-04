#![feature(maybe_uninit_slice)]

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
    pub fn new(joint1_index: u8, joint2_index: u8, length: f32) -> Self {
        Connection(TPE_Connection {
            joint1: joint1_index,
            joint2: joint2_index,
            length: (length / 512.0) as u16,
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

    /// True if any forces are working on the body
    pub fn is_active(&self) -> bool {
        unsafe { TPE_bodyIsActive(&self.0) != 0 }
    }

    /** Adds angular velocity to a soft body. The rotation vector specifies the axis
        of rotation by its direction and angular velocity by its magnitude (magnitude
        of 1.0 will add linear velocity of 2 PI
        per tick to a point in the distance of 1.0 from the
        rotation axis). */
    pub fn spin(&mut self, rotation: Vec3) {
        unsafe { TPE_bodySpin(&mut self.0, rotation.to_internal()) };
    }

    /// Similar to spin but around a different center
    pub fn spin_with_center(&mut self, rotation: Vec3, center: Vec3) {
        unsafe { TPE_bodySpinWithCenter(&mut self.0, rotation.to_internal(), center.to_internal()) };
    }

    /** Instantly rotates a body about an axis (see library conventions for
    the rotation format). */
    pub fn rotate_by_axis(&mut self, rotation: Vec3) {
        unsafe { TPE_bodyRotateByAxis(&mut self.0, rotation.to_internal()) };
    }

    /// Gets rotation of body. Only works if there are at least three joints!
    pub fn rotation(&self) -> Vec3 {
        assert!(self.0.jointCount >= 3);
        Vec3::from_internal(unsafe { TPE_bodyGetRotation(&self.0, 0, 1, 2) })
    }

    /// Moves a body by a certain offset
    pub fn move_by(&mut self, offset: Vec3) {
        unsafe { TPE_bodyMoveBy(&mut self.0, offset.to_internal())};
    }

    /// Moves a body (its center of mass) to a given position
    pub fn move_to(&mut self, position: Vec3) {
        unsafe { TPE_bodyMoveTo(&mut self.0, position.to_internal())};
    }

    /// Adds a velcoity to a soft body
    pub fn accelerate(&mut self, velocity: Vec3) {
        unsafe { TPE_bodyAccelerate(&mut self.0, velocity.to_internal())};
    }

    /// Untested...
    pub fn make_box(width: f32, depth: f32, height: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 8] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 16] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeBox(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(width).to_internal(),
            Unit(depth).to_internal(),
            Unit(height).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 8]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 16]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }

    /// Untested...
    pub fn make_center_box(width: f32, depth: f32, height: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 9] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 18] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeCenterBox(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(width).to_internal(),
            Unit(depth).to_internal(),
            Unit(height).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 9]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 18]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }
    /// Untested...
    pub fn make_rect(width: f32, depth: f32, height: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 4] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 6] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeCenterBox(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(width).to_internal(),
            Unit(depth).to_internal(),
            Unit(height).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 4]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 6]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }

    /// Untested...
    pub fn make_center_rect(width: f32, depth: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 5] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 10] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeCenterRect(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(width).to_internal(),
            Unit(depth).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 5]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 10]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }

    /// Untested...
    pub fn make_center_rect_full(width: f32, depth: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 5] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 10] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeCenterRectFull(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(width).to_internal(),
            Unit(depth).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 5]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 10]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }

    /// Untested...
    pub fn make_triangle(side_length: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 3] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 3] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_makeTriangle(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(side_length).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 3]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 3]>(connections) };
        (joints.to_vec(), connections.to_vec())
    }


    /// Untested...
    pub fn make2line(length: f32, joint_size: f32) -> (Vec<Joint>, Vec<Connection>) {
        let mut joints : [MaybeUninit<TPE_Joint>; 2] = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut connections : [MaybeUninit<TPE_Connection>; 1] = unsafe { MaybeUninit::zeroed().assume_init() };
        unsafe { TPE_make2Line(
            MaybeUninit::slice_as_mut_ptr(&mut joints),
            MaybeUninit::slice_as_mut_ptr(&mut connections),
            Unit(length).to_internal(),
            Unit(joint_size).to_internal()
        ) };
        let joints = unsafe { core::mem::transmute::<_, [Joint; 2]>(joints) };
        let connections = unsafe { core::mem::transmute::<_, [Connection; 1]>(connections) };
        (joints.to_vec(), connections.to_vec())
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

    /// Add a body to the world.
    /// Removing a body (or altering its joints/connections) is intentionally not allowed
    ///
    /// Returns the index that the body was added as
    #[must_use]
    pub fn add_body(&mut self, joints: Vec<Joint>, connections: Vec<Connection>, mass: f32) -> usize {
        let joints = self.joints_arena.alloc(joints);
        let connections = self.connections_arena.alloc(connections);
        let body = Body::new(joints, connections, mass);
        self.bodies_vec.push(body);
        self.fix_world_bodies_ptr();
        self.bodies_vec.len() - 1
    }

    fn fix_world_bodies_ptr(&mut self) {
        let ptr: *mut TPE_Body = unsafe { core::mem::transmute(self.bodies_vec.as_mut_ptr())};
        self.world.0.bodies = ptr;
        self.world.0.bodyCount = self.bodies_vec.len() as u16;
    }

    /// Simulate one step of the world physics
    pub fn step(&mut self) {
        self.world.step();
    }

    /// Access a single body so you can call body-mutating functions on it
    pub fn get_body(&mut self, body_index: usize) -> &mut Body {
        &mut self.bodies_vec[body_index]
    }

    pub fn bodies_iter(&mut self) -> impl Iterator<Item = &mut Body> {
        self.bodies_vec.iter_mut()
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;
}
