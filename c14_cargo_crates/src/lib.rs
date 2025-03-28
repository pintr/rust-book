//! # c14_cargo_crates
//!
//! `c14_cargo_crates` is a package that exaplain how crates and docs work

/// Adds one to the number given.
///
/// # Arguments
///
/// * `x: i32` - The number given.
///
/// # Returns
///
/// * `i32`: The result of the sum.
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = c14_cargo_crates::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
// Those above are common sections of teh docs, additional are:
// - Panics: scenarios in which the function might panic
// - Errors if the function returns a `Result`
// - Safety if the function is unsafe (chapter 20)
// If the Examples section contains code it will be run as test with `cargo test`
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod art {
    //! # Art
    //!
    //! A library for modeling artistic concepts.

    // Add `pub use` to re-export the items at top level
    pub use self::kinds::PrimaryColor;
    pub use self::kinds::SecondaryColor;
    pub use self::utils::mix;

    pub mod kinds {
        /// The primary colors according to the RYB color model.
        #[derive(Debug)]
        pub enum PrimaryColor {
            Red,
            Yellow,
            Blue,
        }

        /// The secondary colors according to the RYB color model.
        #[derive(Debug)]
        pub enum SecondaryColor {
            Orange,
            Green,
            Purple,
        }
    }

    pub mod utils {
        use crate::art::kinds::*;

        /// Combines two primary colors in equal amounts to create a secondary color.
        /// By default it returns Orange
        pub fn mix(c1: &PrimaryColor, c2: &PrimaryColor) -> SecondaryColor {
            match (c1, c2) {
                (PrimaryColor::Red, PrimaryColor::Yellow)
                | (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
                (PrimaryColor::Red, PrimaryColor::Blue)
                | (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
                (PrimaryColor::Blue, PrimaryColor::Yellow)
                | (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
                _ => SecondaryColor::Orange,
            }
        }
    }
}
