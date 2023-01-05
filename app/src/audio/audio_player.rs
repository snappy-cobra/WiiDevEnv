use alloc::sync::Arc;
use ogc_rs::asnd::Asnd;
use super::ogg_player::OGGPlayer;
use super::audio::{Audio, AudioFormat};

/**
 * General audio player interface, hiding the specific audio format players.
 */
pub struct AudioPlayer {
    asnd: Arc<Asnd>,
    ogg_player: OGGPlayer,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        let asnd = Arc::new(Asnd::init());
        return AudioPlayer {
            asnd: asnd.clone(),
            ogg_player: OGGPlayer::new(asnd.clone()),
        };
    }

    /**
     * Play a given sound.
     */
    pub fn play(&mut self, audio: &Audio) {
        match audio.get_format() {
            AudioFormat::OGG => self.ogg_player.play(audio),
            AudioFormat::MP3 => () // TODO : Not supported yet due to asnd sharing issue.
        }
    }

    /**
     * Stop the given sound.
     */
    pub fn stop(&mut self, audio: &Audio) {
        match audio.get_format() {
            AudioFormat::OGG => self.ogg_player.stop(audio),
            AudioFormat::MP3 => () // TODO : Not supported yet due to asnd sharing issue.
        }
    }

    /**
     * Set the global audio volume.
     */
    pub fn set_volume(&mut self, volume: u32) {
        self.ogg_player.set_volume(volume);
        // TODO : MP3 not supported yet due to asnd sharing issue.
    }
}