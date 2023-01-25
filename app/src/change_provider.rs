/**
 * Wii specific implementation of a change provider.
 */
pub struct WiiChangeProvider {
    earlier: &Instant,
    input_manager: InputManager
}

impl WiiChangeProvider {
    pub fn new(input_manager: InputManager) -> WiiChangeProvider {
        return WiiChangeProvider {
            earlier: Instant::now(),
            input_manager
        };
    }

    /// Given the Instant returned the last time this function was called,
    /// will calculate the duration elapsed since then, and returns a new Instant
    /// to be passed back to this function next iteration.
    fn calculate_delta_time(&self) -> (Duration, Instant) {
        let now = Instant::now();
        let delta_time = Self::elapsed_between(self.earlier, &now);
        (delta_time, now)
    }

    /// Calculate the `Duration` between two different `Instant`s
    fn elapsed_between(start: &Instant, end: &Instant) -> Duration {
        Duration::from_nanos(Instant::from_ticks(end.ticks - start.ticks).nanosecs())
    }
}

impl ChangeProvider for WiiChangeProvider {
    fn get_changes(&mut self) -> Changes {
        let (delta_time, now) = self.calculate_delta_time();
        self.earlier = now;
        return Changes {
            controls: self.input_manager.update(),
            delta_time = delta_time
        }
    }
} 