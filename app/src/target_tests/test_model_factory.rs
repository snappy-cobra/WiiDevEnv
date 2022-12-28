use hashbrown::HashMap;
use crate::renderer::model_factory::ModelFactory;
use crate::raw_data_store::AssetName;

/**
 * Test if the model factory behaves as expected.
 */
pub fn test_model_factory(tests: &mut HashMap<&'static str, fn()>) {
    tests.insert("Loading a textured cube.", test_textured_cube);
}

/**
 * Test if we load a valid textured cube.
 */
fn test_textured_cube() {
    let mut factory = ModelFactory::new();
    factory.load_models();
    let textured_model = factory.get_model(AssetName::Cube).unwrap();

    // Check if the vertices are non-empty and a multiple of 3 (X, Y, Z).
    assert!(textured_model.model.vertices.len() > 0);
    assert!(textured_model.model.vertices.len() % 3 == 0);

    // Check if the indexes are non-empty
    assert!(textured_model.model.indices.len() > 0);

    // Check if the texture coords are present and are length of 2 (U, V) times vertex count.
    assert!(textured_model.model.tex_coords.len() > 0);
    let num_vertices = textured_model.model.vertices.len() / 3;
    assert!(textured_model.model.tex_coords.len() == num_vertices * 2);
}