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
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl WiiMoteControl {
    pub fn nothing() -> WiiMoteControl {
        return WiiMoteControl {
            home_button_down: false,
            one_button_down: false,
        }
    }
}