
#[derive(Debug)]
pub struct SphereCollider {
    pub radius: f32,
    pub gravity: bool,
    pub has_been_registered: bool,
}

#[derive(Debug)]
pub struct Rigidbody {
    pub mass: f32,
}