use alloc::vec::Vec;
use wavefront::{Obj};
use alloc::collections::BTreeMap;
use super::model_factory::{ModelFactory};

/**
 * Our representation of a model.
 */
pub struct IndexedModel {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>
}

/**
 * Data structure for the model factory.
 */
pub struct IndexedModelFactory {
    model_factory : ModelFactory,
    indexed_models : BTreeMap<&'static str, IndexedModel>
}

/**
 * Implementation of the model factory: allows for preloading and fetching model data.
 */
impl IndexedModelFactory {
    /**
     * Create a new factory.
     */
    pub fn new() -> IndexedModelFactory {
        IndexedModelFactory {
            model_factory: ModelFactory::new(),
            indexed_models: BTreeMap::new()
        }
    }

    /**
     * Load all models.
     */
    pub fn load_models(&mut self) {
        self.model_factory.load_models();
        self.indexed_models.clear();
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&mut self, key: &'static str) -> Option<&IndexedModel> {
        if !self.indexed_models.contains_key(key) {
            match self.model_factory.get_model(key) {
                Some(model) => self.indexed_models.insert(key, IndexedModelFactory::index_model(model)),
                None => return None
            };
        }
        return self.indexed_models.get(key);
    }

    /**
     * Turn a model into its indexed equivalent
     */
    fn index_model(model: &Obj) -> IndexedModel {
        let mut vertex_map: Vec<&Vertex> = Vec::new(); // ?
        let mut vertex_vec: Vec<[f32; 3]> = Vec::new();
        let mut index_vec: Vec<u16> = Vec::new();
        for vertex in model.positions() {
            match vertex_vec.iter().position(|r| r[0] == vertex[0] && r[1] == vertex[1] && r[2] == vertex[2]) {
                Some(index) => index_vec.push(index as u16),
                None => vertex_vec.push(vertex)
            }
        }
        return IndexedModel {vertices: vertex_vec.flatten().to_vec(), indices: index_vec};
    }
}