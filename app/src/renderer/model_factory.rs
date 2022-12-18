use alloc::vec::Vec;
use ogc_rs::print;
use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use wavefront::Obj;

use crate::raw_data_store::RawDataStore;

use super::indexed_model::IndexedModel;

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models : BTreeMap<&'static str, IndexedModel>
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
            models: BTreeMap::new()
        }
    }

    /**
     * Load all models.
     */
    pub fn load_models(&mut self) {
        for entry in RawDataStore::entries() {
            let key = entry.0;
            let raw_data = entry.1;
            let string_data = from_utf8(raw_data).unwrap();
            match Obj::from_lines(string_data.lines()) {
                Ok(object) => {
                    self.models.insert(key, IndexedModel::new(&object));
                },
                Err(error) =>{
                    print!("Error loading model: {}", error);
                }
            }
        }
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: &'static str) -> Option<&mut IndexedModel> {
        return self.models.get_mut(key);
    }
}