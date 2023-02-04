/**
 * Enumerates all models that exist in the project.
 * Each of them can be turned into its actual raw data by calling `to_data()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum AssetName {
    Cube,
    Suzanne,
    Triangle,
    Potato,
    Plate,
    CubeTexture,
    TriangleTexture,
    SuzanneTexture,
    PotatoTexture,
    PlateTexture,
    DemoMusic,
    BoingSFX
}

impl AssetName {
    /// Returns the raw model data of this particular ModelName
    ///
    /// Internally, `include_bytes!` is used
    /// so each of the files in the ../data directory is included at compile time.
    pub const fn to_data(&self) -> &'static [u8] {
        match self {
            AssetName::Cube => include_bytes!("data/Cube.obj"),
            AssetName::Suzanne => include_bytes!("data/Suz.obj"),
            AssetName::Triangle => include_bytes!("data/Tri.obj"),
            AssetName::Potato => include_bytes!("data/Potato.obj"),
            AssetName::Plate => include_bytes!("data/Plate.obj"),
            AssetName::CubeTexture => include_bytes!("data/Cube.png"),
            AssetName::TriangleTexture => include_bytes!("data/Tri.png"),
            AssetName::SuzanneTexture => include_bytes!("data/Suz.png"),
            AssetName::PotatoTexture => include_bytes!("data/Potato.png"),
            AssetName::PlateTexture => include_bytes!("data/Plate.png"),
            AssetName::DemoMusic => include_bytes!("data/DemoMusic.ogg"),
            AssetName::BoingSFX => include_bytes!("data/Boing.ogg"),
        }
    }
}
