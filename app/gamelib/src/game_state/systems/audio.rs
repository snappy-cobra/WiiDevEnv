use crate::game_state::{GameState, components::audio::Audio};
use hecs::Entity;
use ogc_rs::prelude::Vec;

pub fn system_play_audio(state: &mut GameState) {
    if state.changes.delta_time.is_zero() {
        return;
    }

    let mut query = state.world.query::<&mut Audio>();
    let audios: Vec<(Entity, &mut Audio)> = query.iter().collect();
    let server_provider = state.server_provider.as_ref().unwrap().borrow_mut();

    server_provider.audio_server.set_volume(100);
    for (_entity, audio) in audios {
        if !audio.is_processed {
            //server_provider.audio_server.play(&AssetName::DemoMusic, PlayMode::Infinite);
            server_provider.audio_server.play(&audio.asset_name, audio.play_mode.clone());
            audio.is_processed = true;
        }
    }
}
