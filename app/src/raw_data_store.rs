/**
 * Enumerates all models that exist in the project.
 * Each of them can be turned into its actual raw data by calling `to_data()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AssetName {
    Suzanne,
    SuzanneTexture,
    Triangle,
    TriangleTexture,
}

impl AssetName {
    /// Returns the raw model data of this particular ModelName
    ///
    /// Internally, `include_bytes!` is used
    /// so each of the files in the ../data directory is included at compile time.
    pub const fn to_data(&self) -> &'static [u8] {
        match self {
            AssetName::Suzanne => include_bytes!("../data/Suz.obj"),
            AssetName::SuzanneTexture => include_bytes!("../data/Suz.png"),
            AssetName::Triangle => include_bytes!("../data/Tri.obj"),
            AssetName::TriangleTexture => include_bytes!("../data/Tri.png"),
        }
    }
}
