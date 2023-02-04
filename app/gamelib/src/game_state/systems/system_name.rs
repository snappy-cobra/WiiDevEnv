use crate::game_state::GameState;
use crate::game_state::systems::physics::*;
use super::audio::system_play_audio;
use super::motion::*;
use super::actions::*;
use super::render::*;
use super::gamemaster::*;

/**
 * Enumerates all systems that exist in the project.
 * Each of them can be turned into its actual function by calling `to_function()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SystemName {
    DebugPhysics,
    GameMaster,
    ExitAction,
    MovingPlatform,
    StopAction,
    ShakeAction,
    IntegrateMotion,
    BounceBounds,
    PlayAudio,
    RenderMeshes,
    RegisterCollider,
    PhysicsToPosition,
    PatatoControl,
}

impl SystemName {
    /// Returns the function of this particular SystemName
    pub const fn to_function(&self) -> &'static dyn Fn(&mut GameState) {
        match self {
            SystemName::DebugPhysics => &system_render_debug_physics,
            SystemName::GameMaster => &system_gamemaster,
            SystemName::ExitAction => &system_exit_action,
            SystemName::MovingPlatform => &system_moving_platform,
            SystemName::StopAction => &system_stop_action,
            SystemName::RegisterCollider => &system_register_collider,
            SystemName::ShakeAction => &system_shake_action,
            SystemName::IntegrateMotion => &system_integrate_motion,
            SystemName::BounceBounds => &system_bounce_bounds,
            SystemName::PlayAudio => &system_play_audio,
            SystemName::RenderMeshes => &system_render_meshes,
            SystemName::PhysicsToPosition => &system_physics_to_position,
            SystemName::PatatoControl => &system_control_potato
        }
    }
}