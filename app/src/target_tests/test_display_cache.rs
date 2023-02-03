use crate::rendering::display_cache::DisplayCache;
use gamelib::data_store::textured_model_name::TexturedModelName;
use hashbrown::HashMap;

/**
 * Test if the model factory behaves as expected.
 */
pub fn test_display_cache(tests: &mut HashMap<&'static str, fn()>) {
    tests.insert(
        "Test creating a display cache list.",
        test_create_cache_list,
    );
}

/**
 * Test if we can create a display cache
 */
fn test_create_cache_list() {
    let mut display_cache = DisplayCache::new();
    let list = display_cache.get_display_list(&TexturedModelName::Cube);
    assert!(!list.is_initialized());
    list.open();
    assert!(!list.is_initialized());
    list.close();
    assert!(list.is_initialized());
    list.open();
    assert!(!list.is_initialized());
    list.close();
    assert!(list.is_initialized());
}
