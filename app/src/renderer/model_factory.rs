use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use alloc::vec::Vec;
use ogc_rs::print;
use wavefront::Obj;

use crate::raw_data_store::ModelName;

use super::indexed_model::IndexedModel;
use super::texture::Texture;
use super::textured_model::TexturedModel;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const MODEL_KEYS: [ModelName; 1] = [ModelName::Suzanne];

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models: BTreeMap<ModelName, TexturedModel>,
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
        for model_name in MODEL_KEYS {
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

            // All went well, insert the textured model.
            self.models.insert(model_name, TexturedModel::new(indexed_model, texture));
        }
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: ModelName) -> Option<&mut TexturedModel> {
        return self.models.get_mut(&key);
    }
}
