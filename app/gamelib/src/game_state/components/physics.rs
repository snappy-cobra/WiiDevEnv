
#[derive(Debug)]
pub struct SphereCollider {
    pub radius: f32,
    pub gravity: bool,
    pub body_index: usize,
    pub has_been_registered: bool,
}