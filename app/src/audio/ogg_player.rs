use crate::raw_data_store::AssetName;
use alloc::sync::Arc;
use libc::c_void;
use ogc_rs::prelude::Asnd;
use ogglib::*;

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
    pub fn new(asnd: Asnd) -> Self {
        Self {
            _asnd: Arc::<Asnd>::new(asnd),
        }
    }

    /**
     * Play the given OGG audio file. Set the audio to looping to play it infinitely.
     */
    pub fn play(&self, audio: &AssetName, is_looping: bool) {
        let buffer = audio.to_data();
        let buffer_length = buffer.len() as i32;
        let buffer_ptr = buffer.as_ptr().cast_mut() as *mut c_void;
        let play_mode: i32 = if is_looping {
            OGG_INFINITE_TIME as i32
        } else {
            OGG_ONE_TIME as i32
        };
        unsafe {
            PlayOgg(buffer_ptr, buffer_length, 0, play_mode);
        }
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
