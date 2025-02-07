/// As a project grows, it's common to have a lot of modules and files.
/// A good organisasions can be achieved by splitting the code
/// The module system in Rust is a way to organise code and control the visibility of code and consists of:
/// - Packages: A Cargo feature that lets you build, test, and share crates
/// - Crates: A tree of modules that produces a library or executable
/// - Modules and use: Let you control the organization, scope, and privacy of paths
/// - Paths: A way of naming an item, such as a struct, function, or module
/// The examples are in src/lib.rs

fn main() {
    packages_crates();
    modules();
    paths();
    use_keyword();
    split_modules();
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

    // In order to work with modules it is important to understand the use of paths, use and pub keywords, how they work in the compiler, and how the code is organized:
    // Crate root: The compiler at first looks for the crate root to compile: src/main.rs for binaries, src/lib.rs for libraries.
    // Declaring modules: Modules are declared with the mod keyword followed by the name of the module and a block containing the definitions of the module.
    //                    When running `mod garden` the compiler looks for the module's code inline, in the file src/garden.rs, or src/garden/mod.rs.
    // Declaring submodules: Submodules are declared in the same way as modules, but they are nested inside the parent module, it follows the same rules as modules.
    //                       Running `mod vegetables` the compiler looks for the module's code inline, in the file src/garden/vegetables.rs, or src/garden/vegetables/mod.rs.
    // Paths: When a module is part of a crate the code is referenceable from anywhere in the same crate (if the privacy rules allows it) using the path to the code.
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
    // ```
    // use crate::garden::vegetables::Asparagus;

    // pub mod garden;

    // fn main() {
    //     let plant = Asparagus {};
    //     println!("I'm growing {plant:?}!");
    // }```
    // The `pub mod garden;` line in main.rs tells the compiler to include the code it finds in `src/garden.rs`:
    // ```
    // #[derive(Debug)]
    // pub struct Asparagus {}```

    // Modules lets users to organize code within a crate into groups for readability and reuse, and even control the privacy of items.
    // the code in a module is private by default, private items can't be accessed from outside the module.
    // For example in the context of a restaurant the front of the house is public, but the back of the house is private, check lib.rs for the example.
    // The code defines a module named front_of_house with two child modules: hosting and serving.
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
    // In order to use a function it is necessary to use the path, absolute or relative, to the function, for example:
    // Absolute: crate::front_of_house::hosting::add_to_waitlist();
    // Relative: front_of_house::hosting::add_to_waitlist();

    // If there is a function, which is part of the public API of a library, such as eat_at_restaurant, it can be called outside if marked with the pub keyword.
    // This funciton can call the add_to_waitlist function from the front_of_house module, both using the absolute and relative paths, since it is in the same crate.
    // The choice between absolute and relative paths depends on the context and the code organization.
    // The correct path isn't enough, the function must be public in order to be called from outside the module.
    // In Rust, everything is private by default to parent modules, so the pub keyword must be used to make the function public.
    // items in a parent module can't use private items inside child modules, but the opposite is possible.
    // This is because a child module hides its implementation details, but it can see the context of its parent.
    // To use a function in a public module, the function needs to be public as well, these privacy rules apply to structs, enums, modules, functions, and methods.
    // In a public library crate the public functions, so the APIs, are the "contract" with the users, and define how they can interact with the code.
    // A package can have both a library and a binary crate
    // Both crates will have the package name, the binary crate will call the code in the library crate.
    // Only the library crate can be shared with other crates, the binary crate is specific to the package.
    // The library crate can be made public, and the binary crate uses the library crate as a dependency.

    // Another way to construct a relative path is to use the super keyword, which refers to the parent module. Such as `..` in a file system. Check lib.rs for the example.
    // In this case super is used to refer to the deliver_order function which is a sibling of the module back_of_house.

    // No only functions can be made public, but structs and enums too.
    // The private parts cannot be accessed from outside the module, only the public parts can.
    // For this reason in eat_at_restaurant the toast field can be accessed, but the seasonal_fruit field cannot.
    // Additionally, since seasonal_fruit is private, we need a public constructor to create a Breakfast instance that sets the seasonal_fruit field.
    // In contrast for an enum if it is public, all its variants are public.
}

fn use_keyword() {
    //! The use keyword brings a path into scope, allowing it to be referenced with a shorter name.
    // The use keyword allows to create a sort of symbolic link to the path, so it can be used without the full path.
    {
        use c7_module_system::eat_at_restaurant;

        c7_module_system::eat_at_restaurant(); // Without the use keyword
        eat_at_restaurant(); // With the use keyword
    }
    // Usually the parent is brought into scope, so, to call a from a child module, the parent module needs to be specified, making it clear that the function isn't local.
    // Additionally, using the parent module, distiguishes elements that may have the same name in different modules.
    {
        use std::collections::HashMap;
        use std::fmt;
        use std::io;

        let mut map = HashMap::new();
        map.insert(1, 2);

        fn _function1() -> fmt::Result {
            Ok(())
        } // Result from fmt module
        fn _function2() -> io::Result<()> {
            Ok(())
        } // Resuklt from io module
    }
    // Another solution to this problem is to use the as keyword to create an alias for the path.
    {
        use std::fmt::Result;
        use std::io::Result as IoResult;

        fn _function1() -> Result {
            Ok(())
        }
        fn _function2() -> IoResult<()> {
            Ok(())
        }
    }
    // When a name is brought into scope with the use keyword, the name is private by default.
    // to enable the use of the name outside of the module, the pub keyword must be used.
    // Re-exporting is useful to keep the code organized and to provide a public API may be different from the internal organization.
    // In this way the structure is mantained, but the public API is different.
    #[allow(dead_code)]
    {
        pub use c7_module_system::front_of_house::hosting;

        pub fn eat_at_restaurant() {
            hosting::add_to_waitlist();
        }
    }
    // The use word is used for bring external packages into scope too.
    // Many external packages, called crates, can be found at crates.io.
    // The standard library std is a crate too, it doesn't need to be included in the Cargo.toml file, but it can be used with the use keyword.
    {
        use rand::Rng;
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(1, rand::thread_rng().gen_range(1..100)); // Add a random number to the map with the key 1
    }
    // If there are multiple items from the same crate, they can be brought into scope with a single use keyword.
    // If two paths share one part which completes one of the two paths, the word `self` can be used to refer to the shared part.
    #[allow(unused_imports)]
    {
        use std::io::{self, Write};
        use std::{cmp::Ordering, collections::HashMap};

        let _map: HashMap<i32, i32> = HashMap::new();
        fn _function1() -> Ordering {
            Ordering::Less
        }
        fn _function2() -> io::Result<()> {
            Ok(())
        }
    }
    // If there is the need to bring all the public items defined in a path into scope, the glob operator `*` can be used.
    #[allow(unused_imports)]
    use std::collections::*;
}

fn split_modules() {
    //! In order to make the code easier to navigate, it may be useful to split the code into multiple files.
    // For example we may want to separate the front_of_house module from the lib.rs file.
    // Additionally the hosting module can be extrated from the front_of_house module into its own file.
    // In this case the front_of_house module is defined in src/front_of_house.rs and the hosting module is defined in src/front_of_house/hosting.rs.
    // This is the new style, the old style is to use a mod.rs file, but the new style is more idiomatic.
    // If both styles are used in the same project, the compiler will throw an error.
}
