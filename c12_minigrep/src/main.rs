//! Recap project: `minigrep` (mini globally search a regular expression and print)
//! Command line tool that interacts with file and command line input/output
//! `minigrep` searches a specified file for a specified string
//! It takes as arguments a file path and a string
//! It reads the file, finds the lines that contain the string, and prints them
//! To run it the command will be: `cargo run -- searchstring example-filename.txt`
//! For example: `cargo run -- frog utils/poem.txt`
//! To improve the program structure it will be splitted into `main.rs` and `lib.rs`
//! As long as the parsing logic is samll, it can remain in main
//! When it gets more complicated, it needs to be moved to lib.rs
//! The main only sets up the configuration and calls lib methods
//!
//! # Overview
//! This crate provides a simplified version of `grep` called `minigrep`
//!
//! # Examples
//! ```
//! cargo run -- body utils/poem.txt
//! ```
// Bring into scope env to read arguments: env::args()
// Process is imported to use `process::exit` to stop the program with the error code.
// `process.exit` is similar to `panic!`, but avoid the extra output
use std::{env, process};

// Import the relevant elements from lib.rs
use c12_minigrep::Config;

fn main() {
    // Turn `args` iterator in a vector of strings
    // The `collect` needs to be annotated because Rust can't infer the kind of collection
    let args: Vec<String> = env::args().collect();
    // Print values for debugging using `dbg!`, the variable is dropped but its value is returned
    // let args = dbg!(_args);
    // `args` contains as the first value the name of the binary
    // The other elements are the arguments provided by the user
    // The parsing of the configs can be done in many ways, using a function
    // or, even better, a constructor for the structure `Config` containing them
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    // Print config
    // println!(
    //     "Searching for '{}' in file '{}'",
    //     config.query, config.file_path
    // );

    // Since `run` returns () in the success case it's not necessary to use `unwrap_or_else`
    // Instead `if let` is used just to check if `run` returns an `Err` to manage it
    if let Err(e) = c12_minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
