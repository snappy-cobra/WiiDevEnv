use alloc::boxed::Box;

use crate::modulator::{Modulator, ModulatorEnv};


/// Simple modulator using a value closure/`Fn`, with frequency and amplitude. The
/// closure receives self, elapsed time (in seconds) and returns a new value.
pub struct Wave {
    pub amplitude: f32,
    pub frequency: f32,

    /// Wave closure, receives self and time in s
    pub wave: Box<dyn Fn(&Wave, f32) -> f32>,

    /// Accumulated microseconds
    pub time: u64,
    /// Current value
    pub value: f32,

    /// Enabling toggle
    pub enabled: bool,
}
