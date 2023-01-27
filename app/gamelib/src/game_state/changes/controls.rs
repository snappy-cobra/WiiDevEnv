/// Represents the state of one or multiple Wii controllers w.r.t. the game
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Controls {
    pub home_button_down: bool,
    pub one_button_down: bool,
}

impl Controls {
    pub fn nothing() -> Controls {
        return Controls {
            home_button_down: false,
            one_button_down: false,
        }
    }
}

// pub struct WiiMoteControl {
//     pub motion: Motion
// }