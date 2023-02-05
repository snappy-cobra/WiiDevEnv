use crate::game_state::components::game::*;

#[cfg(feature = "wii")]
use ogc_rs::prelude::Vec;
#[cfg(feature = "wii")]
use ogc_rs::prelude::vec;
#[cfg(not(feature = "wii"))]
use alloc::vec::Vec;
#[cfg(not(feature = "wii"))]
use alloc::vec;
/// Represents the state of one or multiple Wii controllers w.r.t. the game
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)

pub struct Controls {
    pub wii_mote_controls: Vec<WiiMoteControl>
}

impl Controls {
    pub fn nothing() -> Controls {
        return Controls {
            wii_mote_controls: vec![WiiMoteControl::nothing(), WiiMoteControl::nothing(), WiiMoteControl::nothing(), WiiMoteControl::nothing()]
        }
    }

    pub fn get_wii_mote_control(&self, controller_assignment: &ControllerAssignment) -> &WiiMoteControl {
        return &self.wii_mote_controls[controller_assignment.id]
    }
}

pub struct WiiMoteControl {
    pub motion: Option<MotionControl>,
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl WiiMoteControl {
    pub fn nothing() -> WiiMoteControl {
        return WiiMoteControl {
            motion: None,
            home_button_down: false,
            one_button_down: false,
        }
    }
}

pub struct MotionControl {
    pub direction: Direction,
    pub started: bool,
    pub ended: bool,
}

/**Represents the direction that the Wii controller is moved.
 * Xp is left
 * Xn is right
 * Yp is to yourself
 * Yn is away from you
 * Zp is up
 * Zn is down
 */
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Xp,
    Xn,
    Yp,
    Yn,
    Zp,
    Zn,
}