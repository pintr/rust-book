/// As a project grows, it's common to have a lot of modules and files.
/// A good organisasions can be achieved by splitting the code
/// The module system in Rust is a way to organise code and control the visibility of code and consists of:
/// - Packages: A Cargo feature that lets you build, test, and share crates
/// - Crates: A tree of modules that produces a library or executable
/// - Modules and use: Let you control the organization, scope, and privacy of paths
/// - Paths: A way of naming an item, such as a struct, function, or module

fn main() {
    println!("Hello, world!");

    packages_crates();
    modules();
    paths();
}

fn packages_crates() {
    //! A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.
    //! A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.

    // A crate is the smallest amount of code that the Rucompiler considers at a time.
    // Even when using rustc instead of cargo the compiler considers that only file a crate.
    // crates can contian modules and other items.
    // Crates come in two forms: binary (programs compiled into an executable, with main funciton) and library ().
    // A binary is a program with a main function that is compiled into an executable.
    // A library doesn't have a main function and can't be compiled into an executable, instead it defines functionalities that other programs can use.
    // A crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

    // A package is one or more crates that provide a set of functionality.
    // A package contains a Cargo.toml file that describes how to build those crates.
    // For instance Cargo itself is a package that contains the binary crates for CLI tools.
    // When running cargo new project, cargo creates a new directory with a Cargo.toml file and a src directory containing a main.rs file.
    // The main.rs file is the crate root of a binary crate with the same name as the package.
    // The src directory also contains a lib.rs file which is the crate root of a library crate with the same name as the package.
}

fn modules() {
    //! Modules are a way to organize code within a crate for readability, reuse, and privacy of items.

    // In order to work with modules is important to understand the use of paths, use and pub keywords, how they work in the compiler, and how the code is organized:
    // Crate root: The compiler at first looks for the crate root to compile: src/main.rs for binaries, src/lib.rs for libraries.
    // Declaring modules: Modules are declared with the mod keyword followed by the name of the module and a block containing the definitions of the module.
    //                    When running `mod garden` the compiler looks for the module's code inline, in the file src/garden.rs, or src/garden/mod.rs.
    // Declaring submodules: Submodules are declared in the same way as modules, but they are nested inside the parent module, it follows the same rules as modules.
    //                       Running `mod vegetables` the compiler looks for the module's code inline, in the file src/garden/vegetables.rs, or src/garden/vegetables/mod.rs.
    // Paths: When a module is part of a crate the code is referenceable  from anywhere in the same crate (if the privacy rules allows it) using the path to the code.
    // Private vs Public: By default, the items in a module are private, but the pub keyword can be used to make them public.
    // Keyword `use`: The use keyword brings a path into scope, allowing it to be referenced with a shorter name.
    // Considering the following project structure:
    // backyard
    // ├── Cargo.lock
    // ├── Cargo.toml
    // └── src
    //     ├── garden
    //     │   └── vegetables.rs
    //     ├── garden.rs
    //     └── main.rs
    // And the main being:
    // ```use crate::garden::vegetables::Asparagus;

    // pub mod garden;

    // fn main() {
    //     let plant = Asparagus {};
    //     println!("I'm growing {plant:?}!");
    // }```
    // The `pub mod garden;` line in main.rs tells the compiler to include the code it finds in `src/garden.rs`:
    // ```#[derive(Debug)]
    // pub struct Asparagus {}```

    // Modules lets users to organize code within a crate into groups for readability and reuse, and even control the privacy of items.
    // the code in a module is private by default, private items can't be accessed from outside the module.
    // For example in the context of a restaurant the front of the house is public, but the back of the house is private:
    // Create a library named restaurant: `cargo new restaurant --lib` with the folowing structure:
    // ```mod front_of_house {
    //     mod hosting {
    //         fn add_to_waitlist() {}

    //         fn seat_at_table() {}
    //     }

    //     mod serving {
    //         fn take_order() {}

    //         fn serve_order() {}

    //         fn take_payment() {}
    //     }
    // }```
    // The code above defines a module named front_of_house with two child modules: hosting and serving.
    // Each child modules contains functions related to the module it belongs to.
    // The child modules at the same level (hosting and serving) are siblings.
    // The parent module is the root of the module tree, and the child modules are leaves.
}

fn paths() {
    //! To show rust where to find an item in a module tree a path is used, the same way a file system uses paths to find files.
    // A path can be absoluteor relative:
    // - Absolute: Full path starting from a crate root by using a crate name or a literal crate.
    // - Relative: Starts from the current module and uses self, super, or an identifier in the current module.
    // Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
}
