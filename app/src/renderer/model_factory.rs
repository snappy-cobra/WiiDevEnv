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
 * Generate indexed model data at compile time.
 */
const fn gen_indexed_data() -> [(&'static str, &'static ConstModel); MODEL_COUNT] {
    let mut const_models: Vec<(&str, ConstModel)> = Vec::new();
    for (key, raw_data) in RAW_MODEL_DATA {
        let mut model = load_model(raw_data);
        const_models.push((key, index_model(model)));
    }
    return const_models;
}

/**
 * Load a model from raw data
 */
const fn load_model(raw_data: &[u8]) -> Obj {
    let string_data = from_utf8(raw_data).unwrap();
    return Obj::from_lines(string_data.lines()).unwrap();
}

/**
 * Turn a model into its indexed equivalent
 */
const fn index_model(model: Obj) -> ConstModel {
    let vertex_vec: Vec<[f32; 3]> = Vec::new();
    let index_vec: Vec<u16> = Vec::new();
    for vertex in model.positions() {
        match vertex_vec.iter().position(|r| r == vertex) {
            Some(index) => index_vec.push(index as u16),
            None => vertex_vec.push(vertex)
        }
    }
    return ConstModel {vertices: vertex_vec.flatten(), indices: index_vec};
}

const INDEXED_MODELS: [(&str, ConstModel); MODEL_COUNT] = gen_indexed_data();

/**
 * Our representation of a model.
 */
pub struct ConstModel {
    vertices: [f32],
    indices: [u16]
}

/**
 * Data structure for the model factory.
 */
pub struct ModelFactory<'a> {
    models : BTreeMap<&'a str, Obj>
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
    pub fn get_model(&mut self, key: &str) -> Option<&wavefront::Obj> {
        return self.models.get(key);
    }
}