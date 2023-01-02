pub mod controls;

use core::time::Duration;
use controls::Controls;

/// Represents any changes made by the outside world during a single frame.
///
/// Its fields are public so they can be filled in from outside the library
/// (before `GameState::update` is called)
#[derive(Debug, Default)]
pub struct Changes {
    pub controls: Controls,
    pub delta_time: Duration,
}

impl Changes {
    pub fn nothing() -> Changes {
        return Changes {
            controls: Controls::nothing(),
            delta_time: Duration::new(0, 0)
        };
    }
}