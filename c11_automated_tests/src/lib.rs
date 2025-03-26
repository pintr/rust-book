//! Correctness in our programs is the extent to which our code does what we intend it to do.
//! Rust is designed with a high degree of concern about the correctness of programs
//! Rust includes support for writing automated software tests.
//! Tests are Rust function that very that the non test code is working as expected
//! They are performed in three actions:
//! 1. Set up data or state
//! 2. Run the code to be tested
//! 3. Assert that the results are what is expected
//! A test is a function annotated with the `test` attribute: `#[test]`
//! Tests are executed by running `cargo test`, which generates a report
//! There is a statistic `measured` for benchmark tests currently available in nightly Rust
//! Each test is run in parallel in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
//! To prevent tests running in parallel it is possible to use the `--test-threads` and set to 1 for a single thread
//! It is possible to see the values for passing tests, instead of just failing, by adding `-- --show-output`
//! Rust allows to run only one specific test by specifying it `cargo test larger_can_hold_smaller`
//! It is also possible to filter tests by specifying a substring, for example `cargo test larger` will run all the tests that has `larger` in the name
//! I Rust there are two main categories of tests:
//! - Unit tests: small and focused on one module at time, can test private interfaces.
//! - Integration tests: external to the library, and uses the library code as any other library, so using the public elements. Tey are in a separated `tests` folder
//! This module only contains unit tests, as they isolate units of code from the rest, and allows to analyse its behaviour
//! The convention is to create a module named `tests` in each file to ccontain the test functions, and annotate the module with `#[cfg(test)]
//! This annotation tells Rust to run that code only when `cargo test` is called, and not `cargo build`
//! They allow to test both public and private functions

// When crating a lib with cargo a test is automatically generated as a template: the `adder`:
pub fn add(left: u64, right: u64) -> u64 {
    // Add function that adds two numbers
    left + right
}

fn _internal_add(left: usize, right: usize) -> usize {
    left + right
}

// Struct representing a rectangle
#[derive(Debug)]
struct _Rectangle {
    width: u32,
    height: u32,
}

impl _Rectangle {
    fn _can_hold(&self, other: &_Rectangle) -> bool {
        // Check whether a rectangle can hold another one
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: usize) -> usize {
    // Function that adds two to any pointer-sized unsigned integer
    a + 2
}

pub fn greeting(_name: &str) -> String {
    // Function that generates a string greeting a person
    // Return the correct string
    // format!("Hello {_name}!")
    // Return the wrong string
    String::from("Hello")
}

pub struct Guess {
    _value: i32,
}

impl Guess {
    pub fn new(_value: i32) -> Guess {
        // The creation of `Guess` panics if the parameters is not between 1 and 100
        // The messages are different between the two conditions
        if _value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {_value}.");
        } else if _value > 100 {
            panic!("Guess value must be less than or equal to 100, got {_value}.");
        }
        // If the value satisfies the condition the Guess is created
        Guess { _value }
    }
}

#[cfg(test)]
mod tests {
    // The `tests` module is a regular module, with the usual visibility rules
    // Since `tests` is an inner module, the code to be tested needs to be brought into the scope of the inner module
    // To make the outer module code available in test the glob `use super::*;` is used
    use super::*;

    #[test]
    fn it_works() {
        // Test of the add function
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     // Test that fails
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = _Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = _Rectangle {
            width: 5,
            height: 1,
        };
        // The `assert!` macro is used  when a condition evaluates to `true`
        // If it is nothing happens and the test is passed, otherwise it panics
        assert!(larger._can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = _Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = _Rectangle {
            width: 5,
            height: 1,
        };

        // The assert can be be used on negated resultss
        assert!(!smaller._can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        // The `assert_eq!` macro comparres two arguments for eqaulity (`==`)
        // It passes when they are, otherwise it panics
        // The opposite is `assert_ne!` (`!=`)
        // In Rust the arguments are called `left` and `right`
        // Usually `left` is the result while `right` is the testing value
        assert_eq!(result, 4);
    }

    #[test]
    fn greeting_contains_name() {
        let res = greeting("Carol");
        // The `assert!` macro allows to add a custom error message as second parameter
        assert!(
            !res.contains("Carol"),
            "Greeting did not contain name, value was `{res}`"
        );
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        // This test uses the attribute `#[should_panic]`
        // The test is passed if the code panics, otherwise it fails
        Guess::new(200);
    }
    #[test]
    #[should_panic(expected = "greater than or equal to 1")]
    fn lower_then_one() {
        // `#[should_panic]` attribute's `expected` parameter is a substring of the message of the panic
        // This is used to properly identify the panic
        Guess::new(0);
    }

    #[test]
    fn it_works_2() -> Result<(), String> {
        // A test can also have the `Result<(), String>` return type
        // It return `Ok` when the test passes, `Err` with string otherwise
        // In this way the question mark operator can be used in the body of tests
        // This is convenient for tests that should fail if any operation returns `Err`
        // This kind of tests doesn't allow `#[should_panic]`
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Ignored test by default by using the `#[ignore]` annotation
        // The ignored tests can be run calling `cargo test -- --ignored`
    }

    #[test]
    fn internal() {
        // Test of a private funciton
        let res = _internal_add(2, 2);
        assert_eq!(res, 4)
    }
}
