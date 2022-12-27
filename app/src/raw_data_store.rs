/**
 * Enumerates all models that exist in the project.
 * Each of them can be turned into its actual raw data by calling `to_data()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ModelName {
    Suzanne,
    SuzanneTexture,
    Triangle,
    TriangleTexture,
}

impl ModelName {
    /// Returns the raw model data of this particular ModelName
    ///
    /// Internally, `include_bytes!` is used
    /// so each of the files in the ../data directory is included at compile time.
    pub const fn to_data(&self) -> &'static [u8] {
        match self {
            ModelName::Suzanne => include_bytes!("../data/Suz.obj"),
            ModelName::SuzanneTexture => include_bytes!("../data/Suz.png"),
            ModelName::Triangle => include_bytes!("../data/Tri.obj"),
            ModelName::TriangleTexture => include_bytes!("../data/Tri.png"),
        }
    }
}
