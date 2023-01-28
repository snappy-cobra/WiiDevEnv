use crate::data_store::asset_name::AssetName;

/**
 * Defines how you want the file to be played.
 */
pub enum PlayMode {
    Infinite,
    OneTime,
}

pub trait AudioServer {
    /**
     * Play the given audio file. Set the audio to looping to play it infinitely.
     */
    fn play(&self, audio: &AssetName, play_mode: PlayMode);
    fn set_volume(&self, volume: u32);
    fn stop(&self);
}
