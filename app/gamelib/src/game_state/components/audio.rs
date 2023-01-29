use crate::{data_store::asset_name::AssetName, servers::audio::PlayMode};

#[derive(Debug)]
pub struct Audio {
    pub asset_name: AssetName,
    pub play_mode: PlayMode,
    pub is_processed: bool
}

impl Audio {
    pub fn new(asset_name: AssetName, play_mode: PlayMode) -> Self {
        Self { asset_name, play_mode, is_processed: false }
    }
}