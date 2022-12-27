use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use alloc::vec::Vec;
use ogc_rs::print;
use wavefront::Obj;

use crate::raw_data_store::ModelName;

use super::indexed_model::IndexedModel;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const MODEL_KEYS: [ModelName; 1] = [ModelName::Suzanne];

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models: BTreeMap<ModelName, IndexedModel>,
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
        for model in MODEL_KEYS {
            let raw_data = model.to_data();
            let string_data = from_utf8(raw_data).unwrap();
            match Obj::from_lines(string_data.lines()) {
                Ok(object) => {
                    self.models.insert(model, IndexedModel::new(&object));
                }
                Err(error) => {
                    print!("Error loading model: {}", error);
                }
            }
        }
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: ModelName) -> Option<&mut IndexedModel> {
        return self.models.get_mut(&key);
    }
}
