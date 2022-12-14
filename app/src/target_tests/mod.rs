use ogc_rs::*;
use hashbrown::HashMap;

pub fn tests() -> HashMap<&'static str, fn()> {
    let mut tests: HashMap<&'static str, fn()> = HashMap::new();

    tests.insert("Trivial test", || {
        assert!(true)
    });

    // tests.insert("Problematic test", || {
    //     assert!(1 == 0)
    // });

    tests
}


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
