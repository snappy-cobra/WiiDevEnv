use alloc::collections::BTreeMap;
use ogc_rs::audio;
use crate::raw_data_store::AssetName;
use alloc::vec::Vec;

/**
 * All audio files must be defined in this list.
 */
const AUDIO_DATA: [(AssetName, AudioFormat); 1] = [
    (AssetName::DemoMusic, AudioFormat::MP3),
];

/**
 * Generic interface for dealing with audio.
 */
pub struct Audio {
    format: AudioFormat,
    buffer:  &'static [u8],
    looping: bool
}

/**
 * Describes teh way the audio should be handled.
 */
#[derive(Clone, Copy)]
pub enum AudioFormat {
    MP3,
    OGG
}

impl Audio {
    pub fn get_format(&self) -> AudioFormat {
        return self.format;
    }

    pub fn get_buffer(&self) -> &'static [u8] {
        return self.buffer;
    }

    pub fn is_looping(&self) -> bool {
        return self.looping;
    }
}

/**
 * Central location for accessing audio instances.
 */
pub struct AudioStore {
    audios: BTreeMap<AssetName, Audio>,
}

impl AudioStore {
    pub fn new() -> AudioStore {
        let mut res: Self = AudioStore {
            audios: Default::default(),
        };
        res.load_audios();
        res
    }

    /**
     * Loads all audio instances.
     */
    fn load_audios(&mut self) {
        let loaded_audios: Vec<(AssetName, Audio)> = AUDIO_DATA
            .into_iter()
            .map(|(audio_name, audio_format)| {
                let audio = Audio {
                    format: audio_format,
                    buffer: audio_name.to_data(),
                    looping: false
                };
                (audio_name, audio)
            })
            .collect();
        self.audios.extend(loaded_audios);
    }

    pub fn get_audio(&self, key: &AssetName) -> Option<&Audio> {
        return self.audios.get(key);
    }

    pub fn get_audio_mut(&mut self, key: &AssetName) -> Option<&mut Audio> {
        return self.audios.get_mut(key);
    }
}