use alloc::sync::Arc;
use ogc_rs::prelude::Asnd;
use super::audio::Audio;
use ogglib::*;
use libc::c_void;

/**
 * OGG audio file player. Interfaces with the ogglib doing the actual work.
 */
pub struct OGGPlayer {
    _asnd: Arc<Asnd>,
}

impl OGGPlayer {
    /**
     * Accept the asnd, as our library will use it and only once can own it at a time.
     */
    pub fn new(asnd: Arc<Asnd>) -> Self {
        Self {
            _asnd: asnd
        }
    }

    /**
     * Play the given OGG audio file. Set the audio to looping to play it infinitly.
     */
    pub fn play(&self, audio: &Audio) {
        let buffer_length = audio.get_buffer().len() as i32;
        let buffer_ptr = audio.get_buffer().as_ptr().cast_mut() as *mut c_void;
        unsafe {
            PlayOgg(buffer_ptr, buffer_length, 0, Self::get_audio_mode(audio));
        }
    }

    /**
     * Returns the appropriate audio mode for the given audio.
     */
    fn get_audio_mode(audio: &Audio) -> i32 {
        if audio.is_looping() {
            return OGG_INFINITE_TIME as i32;
        } else { 
            return OGG_ONE_TIME as i32;
        };
    }

    pub fn set_volume(&self, volume: u32) {
        unsafe {
            SetVolumeOgg(volume as i32);
        }
    }

    pub fn stop(&self) {
        unsafe {
            StopOgg();
        }
    }
}