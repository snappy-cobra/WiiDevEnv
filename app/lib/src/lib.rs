// in Wii mode, enable `no_std` and import ogc_rs replacements of common functions.
#![cfg_attr(feature = "wii", no_std)]
#[cfg(feature = "wii")]
pub use ogc_rs::{print, println};

#[cfg(not(feature = "wii"))]
pub use std::{print, println};


fn foo() -> bool {
    println!("Hello, world!");
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_foo() {
        let result = super::foo();
        assert!(result);
    }
}
