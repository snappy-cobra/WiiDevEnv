use super::texture::Texture;
use super::indexed_model::IndexedModel;

/**
 * Represents an indexed model that is textured.
 */
pub struct TexturedModel {
    pub model: IndexedModel,
    pub texture: Texture
}

/**
 * Implementation of the textured model.
 */
impl TexturedModel {
    pub fn new(model: IndexedModel, texture: Texture) -> TexturedModel {
        return TexturedModel { model, texture };
    }
}