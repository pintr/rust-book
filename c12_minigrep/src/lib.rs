//! Module containing all the elements necessary for `minigrep` to work, with their tests
// Error is a trait representing the basic expectations for error values
use std::error::Error;
// The `fs` module of `std` is used to handle files
use std::{env, fs};

/// Struct used for collecting the `query` and `file_path` configs
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
    pub fn build_old(args: &[String]) -> Result<Config, &'static str> {
        // This method can be improved using iterators, as follows
        // Error handling return Result with an error if the parameters are not enough
        if args.len() < 3 {
            return Err("not enough parameters");
        }
        // We want to clone the values so Config will own them, without managing lifetimes
        // However the clone function is inefficient
        let query = args[1].clone();
        let file_path = args[2].clone();
        // Read the ignore_case value from the environment, it returns true only if the result is Ok
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    /// Parse `query` and `file_path` and set them as Config parameters
    ///
    /// # Arguments
    ///
    /// * `mut args: impl Iterator<Item = String>` - The arguments as a an element that implements Iterator on strings.
    ///
    /// # Returns
    ///
    /// * `Result<Config, &'static str>`: a Result with the config or a string as error
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // Name of the program

        let query = match args.next() {
            // The value is extracted from the iterator using a `match`
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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

    // Add lines to res. Pay attention not to put semicolon inside of `if` and `else`
    let res = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    // Print each line of the result
    for line in res {
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
    // let mut res = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // The precedent code can be improved using iterators:
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Read the content of the file, and perform the `grep` operation without case
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
/// let query = "rUsT";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
///
/// assert_eq!(
///     vec!["Rust:", "Trust me."],
///     c12_minigrep::search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    /// Tests module used for test-driven development (TDD) with following steps:
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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
