//! Packages from https://crates.io can be used as dependencies of a project
//! It is possible to create and publish any own package to the registry at https://crates.io
//! Rust and Cargo have features to publich packages easily
//! Before publishing it is necessary to document the package using the documentation comments (lib.rs):
//! - `///` is used to describe the items following the comment, such as functions, methods, and structs
//! - `//!` is used to describe the item that contains it, such as crates, packages, and modules
//! To generate the documentation the command is `cargo doc`, it allows multiple parameters such as:
//! - `--open` to open it in the default text/html viewer
//! - `-p, --package` to generate the documention of a specific package
//! - `--workspace` to generate the documentation of the whole workspace
//! The structure of a crate could be convient for developing it but not for the user
//! For this reason it possible to re-export items to make a public structure different from the private one by using `pub use` (lib.rs)
//! TO publish a crate it is necessary to have a https://crates.io account and create an API key at https://crates.io/me
//! Then it is needed to login: `cargo login` using the token
// use c14_cargo_crates::art::kinds::PrimaryColor;
// use c14_cargo_crates::art::utils::mix;
// Use of the art package, a user needs to figure out that `PrimaryColor` is in `kinds`, while `mix` in `utils`
// The structure of `art` is more relevant to developers than to users
// It is possible to remove the internal structure using `pub use` to export items at top level
use c14_cargo_crates::art::PrimaryColor;
use c14_cargo_crates::art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let res = mix(&red, &yellow);
    println!("{:?} + {:?} = {:?}", red, yellow, res);

    // Use of a dependency in the same package
    let n = 1;
    let res = c11_automated_tests::add_two(n);
    println!("{n} + 2 = {res}")
}
