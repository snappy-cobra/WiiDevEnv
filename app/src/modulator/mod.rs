pub mod sources;

use hashbrown::HashMap;
use core::any::Any;
use alloc::boxed::Box;
use core::time::Duration;
use crate::alloc::string::{String, ToString};


/// Modulators provide animated modulation. T is the generic type of the modulator value.
pub trait Modulator<T> {
    /// Value of the modulator at the current time.
    fn value(&self) -> T;

    /// Move the modulator ahead by dt microseconds.
    fn advance(&mut self, dt: u64);

    /// Total accumulated microseconds for the modulator.
    fn elapsed_us(&self) -> u64;

    /// Check if the modulator is disabled
    fn enabled(&self) -> bool;
    /// Toggle enabling/disabling the modulator
    fn set_enabled(&mut self, enabled: bool);

    /// Allow donwcasting.
    fn as_any(&mut self) -> &mut dyn Any;

    /// Range of the modulator as min..=max, or None if the range is indeterminate.
    fn range(&self) -> Option<[T; 2]> {
        None
    }

    /// Current goal of the modulator, or None if not applicable.
    fn goal(&self) -> Option<T> {
        None
    }
    /// Set a goal for the modulator to move towards, if possible.
    fn set_goal(&mut self, _goal: T) {}
}


/// A host for modulators, homogeneous in type T for the value of its modulators,
/// stored in a HashMap for convenience and rapid prototyping.
#[derive(Default)]
pub struct ModulatorEnv<T> {
    mods: HashMap<String, Box<dyn Modulator<T>>>, // live modulators
}

impl<T: Default> ModulatorEnv<T> {
    /// Create an empty ModulatorEnv
    pub fn new() -> Self {
        ModulatorEnv {
            mods: HashMap::default(),
        }
    }

    /// Given a unique key for the modulator, take ownership and hash it into mods table
    pub fn take(&mut self, key: &str, modulator: Box<dyn Modulator<T>>) {
        self.mods.insert(key.to_string(), modulator);
    }
    /// Remove the modulator with given key, let it die
    pub fn kill(&mut self, key: &str) {
        self.mods.remove(key); // ignore the return, let the value die
    }

    /// Take an immutable reference to the mods table
    pub fn get_mods(&self) -> &HashMap<String, Box<dyn Modulator<T>>> {
        &self.mods
    }

    /// Try to fetch an immutable reference to the modulator with the given key
    pub fn get(&self, key: &str) -> Option<&Box<dyn Modulator<T>>> {
        self.mods.get(key)
    }
    /// Try to fetch an mutable reference to the modulator with the given key
    pub fn get_mut(&mut self, key: &str) -> Option<&mut Box<dyn Modulator<T>>> {
        self.mods.get_mut(key)
    }

    /// Return the current value of the given modulator
    pub fn value(&self, key: &str) -> T {
        match self.get(key) {
            Some(modulator) if modulator.enabled() => modulator.value(),
            Some(_) => T::default(),
            None => T::default(),
        }
    }

    /// Return the range of the given modulator
    pub fn range(&self, key: &str) -> Option<[T; 2]> {
        match self.get(key) {
            Some(modulator) if modulator.enabled() => modulator.range(),
            _ => None,
        }
    }
    /// Return the current goal of the given modulator
    pub fn goal(&self, key: &str) -> Option<T> {
        match self.get(key) {
            Some(modulator) if modulator.enabled() => modulator.goal(),
            _ => None,
        }
    }

    /// Return the current value of the given modulator
    pub fn elapsed_us(&self, key: &str) -> u64 {
        match self.get(key) {
            Some(modulator) => modulator.elapsed_us(),
            None => 0,
        }
    }

    /// Advance all owned modulators by dt microseconds
    pub fn advance(&mut self, dt: u64) {
        for v in self.mods.values_mut() {
            if v.enabled() {
                v.advance(dt);
            }
        }
    }

    /// Convert a duration (secs+nanosecs) into total microseconds
    pub fn duration_to_micros(time: Duration) -> u64 {
        time.as_secs() * 1_000_000_u64 + u64::from(time.subsec_nanos()) / 1000_u64
    }
    /// Convert microseconds into floating point seconds
    pub fn micros_to_secs(us: u64) -> f32 {
        us as f32 / 1.0e6_f32
    }
    /// Convert a duration (secs+nanosecs) into floating point seconds
    pub fn duration_to_secs(time: Duration) -> f32 {
        Self::micros_to_secs(Self::duration_to_micros(time))
    }
}
