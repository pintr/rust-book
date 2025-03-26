//! In Rust integration tests are entirely external to the library.
//! They use the library as any other code would, so use only public APIs.
//! There can be as many test files as wanted, and each are different crates
//! In each test file the functions needs to be imported in the test crate's scope
//! Rust treats the `tests` directory specially, so `#[cfg(test)]` is not needed
//! The output of tests have three sections: unit tests, integration tests, and the doc tests
//! It is possible to run run all the tests in a particular file with `cargo test --test file`
//! The tests directory only works for library crates.
use c11_automated_tests::add_two;

mod common;

#[test]
fn it_adds_two() {
    // Use of the setup function of the common module in tests
    common::setup();
    // Test of the public function `add_two` in lib.rs
    let result = add_two(2);
    assert_eq!(result, 4);
}
