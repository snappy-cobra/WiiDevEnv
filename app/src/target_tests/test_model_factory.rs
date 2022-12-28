use hashbrown::HashMap;
use crate::renderer::model_factory::ModelFactory;
use crate::raw_data_store::AssetName;

/**
 * Test if the model factory behaves as expected.
 */
pub fn test_model_factory(tests: &mut HashMap<&'static str, fn()>) {
    tests.insert("Cube vertices list is nonempty.", test_textured_cube);
}

fn test_textured_cube() {
    let mut factory = ModelFactory::new();
    factory.load_models();
    let textured_model = factory.get_model(AssetName::Cube).unwrap();
    assert!(textured_model.model.vertices.len() > 0);
}