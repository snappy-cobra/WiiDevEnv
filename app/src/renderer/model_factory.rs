use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use alloc::vec::Vec;
use ogc_rs::print;
use wavefront::Obj;

use crate::raw_data_store::AssetName;

use super::indexed_model::IndexedModel;
use super::texture::Texture;
use super::textured_model::TexturedModel;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const TEXTURED_MODEL_DATA: [(AssetName, AssetName); 2] = [
    (AssetName::Cube, AssetName::CubeTexture),
    (AssetName::Triangle, AssetName::TriangleTexture)
];

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models: BTreeMap<AssetName, TexturedModel>,
}

/**
 * Implementation of the model factory: allows for preloading and fetching model data.
 */
impl ModelFactory {
    /**
     * Create a new factory.
     */
    pub fn new() -> ModelFactory {
        ModelFactory {
            models: BTreeMap::new(),
        }
    }

    /**
     * Load all models.
     */
    pub fn load_models(&mut self) {
        for (model_name, texture_name) in TEXTURED_MODEL_DATA {
            let raw_data = model_name.to_data();
            let string_data = from_utf8(raw_data).unwrap();

            // Load the model
            let indexed_model: IndexedModel;
            match Obj::from_lines(string_data.lines()) {
                Ok(object) => {
                    indexed_model = IndexedModel::new(&object);
                }
                Err(error) => {
                    print!("Error loading model: {}", error);
                    continue;
                }
            }

            // Load the texture
            let tex_vec: Vec<u8> = Vec::from(texture_name.to_data());
            let texture: Texture;
            match Texture::from_bytes(tex_vec) {
                Ok(new_texture) => texture = new_texture,
                Err(error) => {
                    print!("Error loading model: {}", error);
                    continue;
                }
            }

            // All went well, insert the textured model.
            self.models.insert(model_name, TexturedModel::new(indexed_model, texture));
        }
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: AssetName) -> Option<&mut TexturedModel> {
        return self.models.get_mut(&key);
    }
}
