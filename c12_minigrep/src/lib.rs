//! Module containing all the elements necessary for `minigrep` to work with their tests
// Error is a trait representing the basic expectations for error values
use std::error::Error;
// The `fs` module of `std` is used to handle files
use std::fs;

/// Struct used for collecting the `query` and `file_path` configs
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    /// Parse `query` and `file_path` and set them as Config parameters
    ///
    /// # Arguments
    ///
    /// * `args: &[String]` - The arguments as a vector of strings.
    ///
    /// # Returns
    ///
    /// * `Result<Config, &'static str>`: a Result with the config or a string as error
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Error handling return Result with an error if the parameters are not enough
        if args.len() < 3 {
            return Err("not enough parameters");
        }
        // We want to clone the values so Config will own them, without managing lifetimes
        // However the clone function is inefficient
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

/// Read the content of the file, and perform the `grep` operation
///
/// # Arguments
///
/// * `config: Config` - The config containing query and file path.
///
/// # Returns
///
/// * `Result<Config, &'static str>`: unit type in the Ok case, a type that implements the `Error` trait in the Err case
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Instead of `expect` `?` is used so it will return the error instead of panicking
    let contents = fs::read_to_string(config.file_path)?;

    // Print each line of the result
    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    Ok(())
}

/// Read the content of the file, and perform the `grep` operation
///
/// # Arguments
///
/// * `config: Config` - The config containing query and file path.
///
/// # Returns
///
/// * `Result<Config, &'static str>`: unit type in the Ok case, a type that implements the `Error` trait in the Err case
///
/// # Examples
/// ```
/// let query = "duct";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], c12_minigrep::search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // It is necessary to define a lifetime `'a` in the signature
    // to indicate that the returned vector should contain string slices that reference slices of the argument `contents`
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }

    res
}

// Tests
#[cfg(test)]
mod tests {
    /// Module used for test-driven development (TDD) with following steps:
    /// 1. Write a test that fails and run it to make sure it fails for the reason you expect.
    /// 2. Write or modify just enough code to make the new test pass.
    /// 3. Refactor the code you just added or changed and make sure the tests continue to pass.
    /// 4. Repeat from step 1!
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
