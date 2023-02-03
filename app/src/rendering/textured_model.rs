use super::indexed_model::IndexedModel;
use super::texture::Texture;

/**
 * Represents an indexed model that is textured.
 */
#[derive(Debug)]
pub struct TexturedModel {
    pub model: IndexedModel,
    pub texture: Texture,
}

/**
 * Implementation of the textured model.
 */
impl TexturedModel {
    pub fn new(model: IndexedModel, texture: Texture) -> TexturedModel {
        return TexturedModel { model, texture };
    }
}
