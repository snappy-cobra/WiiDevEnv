use hashbrown::HashMap;
use ogc_rs::*;

pub fn tests() -> HashMap<&'static str, fn()> {
    let mut tests: HashMap<&'static str, fn()> = HashMap::new();

    tests.insert("Trivial test", || {
        let myvar = 1;
        let expected = 1;
        assert_eq!(myvar, expected);
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
