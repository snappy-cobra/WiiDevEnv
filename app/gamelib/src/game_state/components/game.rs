#[derive(Debug)]
pub struct Platform {
}

pub enum OnAnimationFinish {
    Despawn
}

pub enum AnimationType {
    Test
}

pub struct Animation {
    pub duration: f32,
    pub past_time: f32,
    pub animation_type: AnimationType,
    pub on_animation_finish: OnAnimationFinish,
}