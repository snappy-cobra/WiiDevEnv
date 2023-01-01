use crate::game_state::systems::system_integrate_motion;
use crate::game_state::GameState;

/**
 * Enumerates all systems that exist in the project.
 * Each of them can be turned into its actual function by calling `to_function()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SystemName {
    ShakeWii,
    BounceBounds,
    IntegrateMotion,
    CheckExit,
    RenderWorld,
}

impl SystemName {
    /// Returns the function of this particular SystemName
    pub const fn to_function(&self) -> fn(&mut GameState) {
        match self {
            SystemName::ShakeWii => system_integrate_motion,
            SystemName::BounceBounds => system_integrate_motion,
            SystemName::IntegrateMotion => system_integrate_motion,
            SystemName::CheckExit => system_integrate_motion,
            SystemName::RenderWorld => system_integrate_motion,
        }
    }

    /// Tries to return a matching system name from the given string.
    pub const fn from_string(string: &str) -> Result<SystemName, &'static str> {
        match string {
            "shake_wii" => Ok(SystemName::ShakeWii),
            "bounce_bounds" => Ok(SystemName::BounceBounds),
            "integrate_motion" => Ok(SystemName::IntegrateMotion),
            "check_exit" => Ok(SystemName::CheckExit),
            "render_world" => Ok(SystemName::RenderWorld),
            _ => Err("System name unknown")
        }
    }
}
