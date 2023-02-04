use crate::data_store::asset_name::AssetName;
use strum_macros::EnumIter;

/**
 * Enumerates all textured models that exist in the project.
 * Each of them can be turned into its associated assets.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, EnumIter)]
pub enum TexturedModelName {
    Cube,
    Triangle,
    Suzanne,
    Potato,
}

impl TexturedModelName {
    /// Returns the assets associations
    pub const fn get_assets(&self) -> (AssetName, AssetName) {
        match self {
            TexturedModelName::Cube => (AssetName::Cube, AssetName::CubeTexture),
            TexturedModelName::Triangle => (AssetName::Triangle, AssetName::TriangleTexture),
            TexturedModelName::Suzanne => (AssetName::Suzanne, AssetName::SuzanneTexture),
            TexturedModelName::Potato => (AssetName::Potato, AssetName::PotatoTexture),
        }
    }
}
