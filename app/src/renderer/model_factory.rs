use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use alloc::vec::Vec;
use ogc_rs::print;
use wavefront::Obj;

use crate::raw_data_store::RawDataStore;

use super::indexed_model::IndexedModel;
use super::texture::Texture;
use super::textured_model::TexturedModel;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const MODEL_KEYS: [&str; 1] = ["Suzanne"];
const KEY_TEXTURE: &str = "TEX";

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models: BTreeMap<&'static str, TexturedModel>,
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
        let mut store = RawDataStore::new();
        for key in MODEL_KEYS {
            // Get the texture
            let texture: Texture;
            match store.get(key + '_' + KEY_TEXTURE) {
                Some(tex_data) => {
                    let mut tex_vec: Vec<u8> = Vec::from(tex_data);
                    texture = Texture::new(tex_vec);
                }
                None => {
                    print!("Error loading texture for key: {}", key);
                    continue;
                }
            }

            // Get the model
            let raw_model_data = store.get(key).unwrap();
            let string_model_data = from_utf8(raw_model_data).unwrap();
            let indexed_model: TexturedModel;
            match Obj::from_lines(string_model_data.lines()) {
                Ok(object) => {
                    indexed_model = IndexedModel::new(&object);
                }
                Err(error) => {
                    print!("Error loading model for key {} : {}", key, error);
                    continue;
                }
            }

            // All went well, insert the textured model.
            self.models.insert(key, TexturedModel::new(indexed_model, texture));
        }
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: &'static str) -> Option<&mut TexturedModel> {
        return self.models.get_mut(key);
    }
}
