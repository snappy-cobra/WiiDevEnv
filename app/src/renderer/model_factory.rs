use alloc::collections::BTreeMap;
use alloc::str::from_utf8;
use ogc_rs::print;
use wavefront::Obj;

/**
 * All models must be defined in this list, which is filled at compile time.
 */
const raw_model_data: [(&str, &'static [u8]); 2] = [
    ("Teapot", include_bytes!("../../data/Teapot.obj")),
    ("Suzanne", include_bytes!("../../data/Suzanne.obj")),
];

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory<'a> {
    models: BTreeMap<&'a str, Obj>,
}

/**
 * Implementation of the model factory: allows for preloading and fetching model data.
 */
impl<'a> ModelFactory<'a> {
    /**
     * Create a new factory.
     */
    pub fn new() -> ModelFactory<'a> {
        ModelFactory {
            models: BTreeMap::new(),
        }
    }

    /**
     * Load all models.
     */
    pub fn load_models(&mut self) {
        for entry in raw_model_data {
            let key = entry.0;
            let raw_data = entry.1;
            let string_data = from_utf8(raw_data).unwrap();
            match Obj::from_lines(string_data.lines()) {
                Ok(value) => {
                    self.models.insert(key, value);
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
    pub fn get_model(&mut self, key: &str) -> Option<&wavefront::Obj> {
        return self.models.get(key);
    }
}
