pub mod audio;
pub mod renderer;

use crate::servers::audio::AudioServer;
use crate::servers::renderer::RenderServer;

#[cfg(feature = "wii")]
use ogc_rs::prelude::Box;
#[cfg(not(feature = "wii"))]
use std::boxed::Box;

pub struct ServerProvider {
    pub audio_server: Box<dyn AudioServer>,
    pub render_server: Box<dyn RenderServer>
}

impl ServerProvider {
    pub fn new(audio_server: impl AudioServer + 'static, render_server: impl RenderServer + 'static) -> Self {
        Self {
            audio_server: Box::new(audio_server),
            render_server: Box::new(render_server)
        }
    }
}