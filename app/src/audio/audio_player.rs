use alloc::sync::Arc;
use ogc_rs::asnd::Asnd;
use super::ogg_player::OGGPlayer;
use super::audio::{Audio, AudioFormat};

/**
 * General audio player interface, hiding the specific audio format players.
 */
pub struct AudioPlayer {
    _asnd: Arc<Asnd>,
    ogg_player: OGGPlayer,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        let asnd = Arc::new(Asnd::init());
        return AudioPlayer {
            _asnd: asnd.clone(),
            ogg_player: OGGPlayer::new(asnd.clone()),
        };
    }

    /**
     * Play a given sound.
     */
    pub fn play(&self, audio: &Audio) {
        match audio.get_format() {
            AudioFormat::OGG => self.ogg_player.play(audio),
            AudioFormat::MP3 => () // TODO : Not supported yet due to asnd sharing issue.
        }
    }

    /**
     * Stop the given sound.
     */
    pub fn stop(&self) {
        self.ogg_player.stop();
        // TODO : MP3 not supported yet due to asnd sharing issue.
    }

    /**
     * Set the global audio volume.
     */
    pub fn set_volume(&self, volume: u32) {
        self.ogg_player.set_volume(volume);
        // TODO : MP3 not supported yet due to asnd sharing issue.
    }
}