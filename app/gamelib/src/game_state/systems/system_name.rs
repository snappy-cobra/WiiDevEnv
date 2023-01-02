use crate::game_state::GameState;
use super::motion::*;
use super::actions::*;

/**
 * Enumerates all systems that exist in the project.
 * Each of them can be turned into its actual function by calling `to_function()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SystemName {
    ExitAction,
    ShakeAction,
    IntegrateMotion,
    BounceBounds,
}

impl SystemName {
    /// Returns the function of this particular SystemName
    pub const fn to_function(&self) -> fn(&mut GameState) {
        match self {
            SystemName::ExitAction => system_exit_action,
            SystemName::ShakeAction => system_shake_action,
            SystemName::IntegrateMotion => system_integrate_motion,
            SystemName::BounceBounds => system_bounce_bounds,
        }
    }
}
