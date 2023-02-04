#[derive(Debug)]
pub struct Platform {
}

pub enum OnAnimationFinish {
    Despawn,
    Repeat,
    Hand2,
    Hand1,
    Hand0,
    Start
}

pub enum AnimationType {
    None,
    Test,
}

pub struct Animation {
    pub duration: f32,
    pub past_time: f32,
    pub animation_type: AnimationType,
    pub on_animation_finish: OnAnimationFinish,
}