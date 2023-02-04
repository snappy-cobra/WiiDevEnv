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
    HandHolding,
    HandOne,
    HandTwo,
    HandThree,
    HandFist,
    Plate,
    FryPan,
    OilBubble,
    OilSea,
    SweetPotato,
    CubeTexture,
    TriangleTexture,
    SuzanneTexture,
    PotatoTexture,
    PlateTexture,
    DemoMusic,
    BoingSFX,
    HandHoldingTexture,
    HandOneTexture,
    HandTwoTexture,
    HandThreeTexture,
    HandFistTexture,
    FryPanTexture,
    OilBubbleTexture,
    OilSeaTexture,
    SweetPotatoTexture,
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
            AssetName::HandHolding => include_bytes!("data/Hand/Hand_holding.obj"),
            AssetName::HandOne => include_bytes!("data/Hand/Hand_one.obj"),
            AssetName::HandTwo => include_bytes!("data/Hand/Hand_two.obj"),
            AssetName::HandThree => include_bytes!("data/Hand/Hand_three.obj"),
            AssetName::HandFist => include_bytes!("data/Hand/Hand_fist.obj"), 
            AssetName::HandHolding => include_bytes!("data/Hand/Hand_holding.obj"),
            AssetName::FryPan => include_bytes!("data/FryPan.obj"),
            AssetName::OilBubble => include_bytes!("data/OilBubble.obj"),
            AssetName::OilSea => include_bytes!("data/OilSea.obj"),
            AssetName::SweetPotato => include_bytes!("data/SweetPotato.obj"),
            AssetName::HandOne => include_bytes!("data/Hand/Hand_one.obj"),
            AssetName::HandTwo => include_bytes!("data/Hand/Hand_two.obj"),
            AssetName::HandThree => include_bytes!("data/Hand/Hand_three.obj"),
            AssetName::HandThree => include_bytes!("data/Hand/Hand_three.obj"),
            AssetName::HandFistTexture => include_bytes!("data/Hand/Hand_fist.png"),
            AssetName::HandHoldingTexture => include_bytes!("data/Hand/Hand_holding.png"),
            AssetName::HandOneTexture => include_bytes!("data/Hand/Hand_one.png"),
            AssetName::HandTwoTexture => include_bytes!("data/Hand/Hand_two.png"),
            AssetName::HandThreeTexture => include_bytes!("data/Hand/Hand_three.png"),
            AssetName::FryPanTexture => include_bytes!("data/FryPan.png"),
            AssetName::OilBubbleTexture => include_bytes!("data/OilBubble.png"),
            AssetName::OilSeaTexture => include_bytes!("data/OilSea.png"),
            AssetName::SweetPotatoTexture => include_bytes!("data/SweetPotato.png"),
        }
    }
}
