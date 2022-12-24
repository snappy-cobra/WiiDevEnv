/**
 * Enumerates all models that exist in the project.
 * Each of them can be turned into its actual raw data by calling `to_data()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum ModelName {
    Suzanne,
    SuzanneMaterial,
    Triangle,
    TriangleMaterial,
}

impl ModelName {
    /// Returns the raw model data of this particular ModelName
    ///
    /// Internally, `include_bytes!` is used
    /// so each of the files in the ../data directory is included at compile time.
    pub fn to_data(&self) -> &'static [u8] {
        match self {
            ModelName::Suzanne => include_bytes!("../data/Suz.obj"),
            ModelName::SuzanneMaterial => include_bytes!("../data/Suz.mtl"),
            ModelName::Triangle => include_bytes!("../data/Tri.obj"),
            ModelName::TriangleMaterial => include_bytes!("../data/Tri.mtl"),
        }
    }
}
