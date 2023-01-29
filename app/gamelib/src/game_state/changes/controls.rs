use alloc::vec::Vec;
use alloc::vec;
/// Represents the state of one or multiple Wii controllers w.r.t. the game
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)

pub struct Controls {
    pub wii_mote_control: Vec<WiiMoteControl>
}

impl Controls {
    pub fn nothing() -> Controls {
        return Controls {
            wii_mote_control: vec![WiiMoteControl::nothing(), WiiMoteControl::nothing(), WiiMoteControl::nothing(), WiiMoteControl::nothing()]
        }
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Xp,
    Xn,
    Yp,
    Yn,
    Zp,
    Zn,
}