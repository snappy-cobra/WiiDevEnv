use alloc::vec::Vec;
use ogc_rs::print;
use wavefront::{Obj};
use alloc::collections::BTreeMap;
use alloc::str::from_utf8;

const MODEL_COUNT: usize = 2;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const RAW_MODEL_DATA: [(&str, &'static[u8]); MODEL_COUNT] = [
    ("Teapot", include_bytes!("../../data/Teapot.obj")),
    ("Suzanne", include_bytes!("../../data/Suzanne.obj"))
];

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory {
    models : BTreeMap<&'static str, Obj>
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
        for entry in RAW_MODEL_DATA {
            let key = entry.0;
            let raw_data = entry.1;
            let string_data = from_utf8(raw_data).unwrap();
            match Obj::from_lines(string_data.lines()) {
                Ok(value) => {
                    self.models.insert(key, value);
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
    pub fn get_model(&mut self, key: &'static str) -> Option<&wavefront::Obj> {
        return self.models.get(key);
    }
}