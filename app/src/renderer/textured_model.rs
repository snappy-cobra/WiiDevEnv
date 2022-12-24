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
    pub fn new(model: TexturedModel, texture: Texture) -> TexturedModel {
        return TexturedModel { model, texture };
    }
}