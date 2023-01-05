use alloc::sync::Arc;
use ogc_rs::prelude::Asnd;
use super::audio::Audio;

/**
 * OGG audio file player.
 */
pub struct OGGPlayer {
    asnd: Arc<Asnd>,
}

impl OGGPlayer {
    pub fn new(asnd: Arc<Asnd>) -> Self {
        Self {
            asnd
        }
    }

    pub fn play(&mut self, audio: &Audio) {
        // TODO
    }

    pub fn set_volume(&mut self, volume: u32) {
        // TODO
    }

    pub fn stop(&mut self, audio: &Audio) {
        // TODO
    }
}