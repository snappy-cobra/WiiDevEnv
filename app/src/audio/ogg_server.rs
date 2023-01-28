use gamelib::data_store::asset_name::AssetName;
use alloc::sync::Arc;
use libc::c_void;
use ogc_rs::prelude::Asnd;
use ogglib::*;
use gamelib::servers::audio::{AudioServer, PlayMode};

/**
 * OGG audio file player. Interfaces with the ogglib doing the actual work.
 */
pub struct WiiOGGServer {
    _asnd: Arc<Asnd>,
}

impl WiiOGGServer {
    /**
     * Accept the asnd, as our library will use it and only once can own it at a time.
     */
    pub fn new(asnd: Asnd) -> Self {
        Self {
            _asnd: Arc::<Asnd>::new(asnd),
        }
    }    

    fn play_mode_to_ogg_mode(play_mode: PlayMode) -> i32 {
        match play_mode {
            PlayMode::Infinite => OGG_INFINITE_TIME as i32,
            PlayMode::OneTime => OGG_ONE_TIME as i32,
        }
    }
}

impl AudioServer for WiiOGGServer {
    /**
     * Play the given OGG audio file. Set the audio to looping to play it infinitely.
     */
    fn play(&self, audio: &AssetName, play_mode: PlayMode) {
        let buffer = audio.to_data();
        let buffer_length = buffer.len() as i32;
        let buffer_ptr = buffer.as_ptr().cast_mut() as *mut c_void;
        unsafe {
            PlayOgg(buffer_ptr, buffer_length, 0, Self::play_mode_to_ogg_mode(play_mode));
        }
    }

    fn set_volume(&self, volume: u32) {
        unsafe {
            SetVolumeOgg(volume as i32);
        }
    }

    fn stop(&self) {
        unsafe {
            StopOgg();
        }
    }
}