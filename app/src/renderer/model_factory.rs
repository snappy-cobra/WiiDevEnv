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
const TEXTURED_MODEL_DATA: [(AssetName, AssetName); 3] = [
    (AssetName::Cube, AssetName::CubeTexture),
    (AssetName::Triangle, AssetName::TriangleTexture),
    (AssetName::Suzanne, AssetName::SuzanneTexture)
];

/**
 * Data structure for the model factory.
 */
#[derive(Debug)]
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
        let mut res: Self = ModelFactory {
            models: Default::default(),
        };
        res.load_models();
        res
    }

    /**
     * Load all models.
     */
    pub fn load_models(&mut self) {
        // for (model_name, texture_name) in TEXTURED_MODEL_DATA {
        //     let indexed_model: IndexedModel = Self::load_indexed_model(model_name);

        //     // Load the texture
        //     let tex_vec: Vec<u8> = Vec::from(texture_name.to_data());
        //     let texture: Texture;
        //     match Texture::from_bytes(tex_vec) {
        //         Ok(new_texture) => texture = new_texture,
        //         Err(error) => {
        //             print!("Error loading model: {}", error);
        //             continue;
        //         }
        //     }

        //     // All went well, insert the textured model.
        //     self.models.insert(model_name, TexturedModel::new(indexed_model, texture));
        // }

        let loaded_models: Vec<(AssetName, TexturedModel)> = TEXTURED_MODEL_DATA
            .into_iter()
            .filter_map(|(model_name, texture_name)| {
                match (Self::load_indexed_model(&model_name).ok(), Self::load_texture(&texture_name).ok()) {
                    (Some(indexed_model), Some(texture)) =>
                        Some((model_name, TexturedModel::new(indexed_model, texture))),
                    _ => {
                        print!("Skipped model because of loading errors.");
                        None
                    },
                }
            })
            .collect();
        self.models.extend(loaded_models);
    }

    /**
     * Load an indexed model from a given asset name.
     */
    fn load_indexed_model(name: &AssetName) -> Result<IndexedModel, &'static str> {
        let raw_data = name.to_data();
        let string_data = from_utf8(raw_data).unwrap();
        match Obj::from_lines(string_data.lines()) {
            Ok(object) => {
                return Ok(IndexedModel::new(&object));
            }
            Err(_error) => {
                return Err("Error loading indexed model from OBJ data.");
            }
        }
    }

    /**
     * Load an indexed model from a given asset name.
     */
    fn load_texture(name: &AssetName) -> Result<Texture, &'static str> {
        let tex_vec: Vec<u8> = Vec::from(name.to_data());
        return Texture::from_bytes(tex_vec);
    }

    /**
     * Return the given model.
     */
    pub fn get_model(&self, key: &AssetName) -> Option<& TexturedModel> {
        return self.models.get(key);
    }
}
