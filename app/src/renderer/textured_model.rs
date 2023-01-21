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

/**
 * Enumerates all textured models that exist in the project.
 * Each of them can be turned into its associated assets.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum TexturedModelName {
    Cube,
    Triangle,
    Suzanne,
}

impl TexturedModelName {
    /// Returns the assets associations
    pub const fn get_assets(&self) -> &'static [u8] {
        match self {
            TexturedModelName::Cube => (AssetName::Cube, AssetName::CubeTexture),
            TexturedModelName::Triangle => (AssetName::Triangle, AssetName::TriangleTexture),
            TexturedModelName::Suzanne => (AssetName::Suzanne, AssetName::SuzanneTexture),
        }
    }
}