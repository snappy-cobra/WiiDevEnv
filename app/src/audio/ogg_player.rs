use alloc::sync::Arc;
use ogc_rs::prelude::Asnd;

/**
 * OGG audio file player.
 */
pub struct OGGPlayer {
    asnd: Arc<Asnd>,
}

impl OGGPlayer {
    pub fn new(asnd: Asnd) -> Self {
        Self {
            asnd: Arc::new(asnd),
        }
    }

    pub fn play(&mut self, audio: &Audio) {
        // TODO
    }

    pub fn volume(&mut self, volume: u32) {
        // TODO
    }

    pub fn stop(&mut self) {
        // TODO
    }
}