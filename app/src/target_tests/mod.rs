use hashbrown::HashMap;
use ogc_rs::*;

mod test_model_factory;
use test_model_factory::test_model_factory;

/**
 * Main list of tests to run.
 */
pub fn tests() -> HashMap<&'static str, fn()> {
    let mut tests: HashMap<&'static str, fn()> = HashMap::new();

    tests.insert("Trivial test", || assert!(true));
    test_model_factory(&mut tests);

    // tests.insert("Problematic test", || {
    //     assert!(1 == 0)
    // });

    tests
}

/**
 * Kickstart the testing suite.
 */
pub fn run_test_suite() -> isize {
    println!("Running tests...");
    for (name, body) in tests().iter() {
        print!("{} ...", name);
        (body)();
        println!("{} ... ok", name);
    }
    println!("Test run successful!");
    0
}
