use core::panic;

/// Rust has many features for error handling, additionally it requires to aknowledge the possibility of errors, so it requires to handle them.
/// There are two errors: recoverable and unrecoverable.
/// Recoverable errors are those that can be handled and the program can continue. It uses the Result<T, E> enum.
/// Unrecoverable errors are those that are not possible to handle and the program must be stopped. It uses the panic! macro.
///

fn main() {
    unrecoverable_errors();
    recoverable_errors();
    when_panic();
}

fn unrecoverable_errors() {
    //! Unrecoverable errors are characterized by the panic! macro.
    //! The panic can be caused by the code, or by explicit call to the panic! macro.
    // When a panic occurs, Rust by default unwinds (walks back) the stack, cleans up the memory, and then exits.
    // Another possibility is to abort without unwinding the stack, which is faster, but the memory is not cleaned up. This will be done by the OS.
    // To switch to abort `panic = 'abort'` must be added to the profile in the [profile] section of the Cargo.toml file.
    // panic!("crash and burn"); // Causes a panic with the message "crash and burn".

    // When the panic occurs the program will print the line where the panic occurred, print the message, and then exit.
    // let v = vec![1, 2, 3, 4];
    // v[99]; // This will cause a panic because the index is out of bounds.
    // In C or C++ this would cause a buffer overflow, which is a security vulnerability.

    // The backtrace helps to identify the cause of the panic. RUST_BACKTRACE=1 must be set in the environment variables.
    // It's enough to set the environment variable in the terminal before running the program: RUST_BACKTRACE=1 cargo run.
    // To print even more information, RUST_BACKTRACE=full can be set.
    // With `cargo build` and `cargo run` debug symbols are enabled byu default
}

fn recoverable_errors() {
    //! Most error aren?t severe enough to require the program to stop entirely. In these cases, Rust has the Result enum
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // T and E are generic parameters, where T is the type of the value returned in case of success, E in case of error.
    // For example opening a inexistent file will return a Result<File, io::Error>.
    {
        use std::fs::File;
        use std::io::ErrorKind;

        let file_result = File::open("hello.txt");

        // let _file = match file_result {
        //     Ok(file) => file,
        //     Err(error) => panic!("Error opening the file: {}", error),
        // };
        // Similarly to Option, Result and its variants have been brought into scope in the prelude.
        // When the result id Ok the file is returned, otherwise the program panics.
        // In the previous example the program panics when File::open fails, independently of the reason.
        // In this case it failed because the file doesn't exist, but if it failed for any other reason the program would panic anyway.
        // For this reason, if we want to manage different errors in different ways, an inner match expression can be used:

        let _file = match file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Error creating the file: {}", error),
                },
                other_error => panic!("Error opening the file: {:?}", other_error),
            },
        };
        // In this case the file is created if it doesn't exist, otherwise the program panics.
        // `io::Error` is a struct provided by the standard library that has a method `kind` that returns an `ErrorKind` enum.
        // The `ErrorKind` enum provide all the `io::Error` variants, allowing to choose the condition to match, and managing all the other errors in a single branch.
        // `mathc` is useful but primitive as well. With closures and combinators it's possible to manage errors in a more concise way:
        let _file = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {error:?}");
                })
            } else {
                panic!("Problem opening the file: {error:?}");
            }
        });
        // This code hase the same behaviour as above, but without math cases.
        let _ = std::fs::remove_file("hello.txt"); // Remove file for the next examples.
    }
    {
        // The `unwrap` is a shortcut of a match where, if the Result value is Ok, unwrap will return the value inside Ok, otherwise it will panic.
        use std::fs::File;

        let _file_result = File::open("hello.txt").unwrap();
        // In this case, since the file doesn't exist, the `unwrap` will make the program panic.

        // Similarly, the `expect` method panics if the Result is an Err, but it allows to specify the panic message.
        let _file_result = File::open("hello.txt").expect("hello.txt not found");
        // Usually, in production expect is more used than unwrap to give more context about why the operation failed.
    }
    {
        // When a funciton calls something that might fail, instead of handling the error, it can return the error to the calling code.
        // This is called "propagating the error", and gives more control to the calling code, where there might be more context about the error:
        use std::fs::File;
        use std::io::{Error, Read};

        fn _read_username() -> Result<String, Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file, // If `File::open` succedes, username_file gets the falue of file of type `File`.
                Err(e) => return Err(e), // If `File::open` fails, instead of panicking, the error will be passed to the calling function.
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username), // If the `read_to_string` succedesthe content will be appended to username, and the calling code will get it wrapped in a Ok
                Err(e) => Err(e), // If it doesn't the calling code will get the the returned error value wrapped in an Err.
            }
        }
        // This function returns a Result so, if it works, it returns a Ok value holding a String, otherwise it returns an Err holding an instance of io::Error.
        // In this case io::Error is chosen because it's the same error returned by `File::open`, and `read_to_string`.
    }
    #[allow(unused_must_use)]
    {
        // The error propagation is very common, for this reason Rust provides the operator `?` to make it easy to handle.
        use std::fs::File;
        use std::io::{Error, Read};

        fn _read_username_v1() -> Result<String, Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username);
            Ok(username)
        }
        // Placing a `?` after works the same ways as the `match` expressions defined to handle the Result values.
        // So if the value of the Result is Ok, the value in it is returned, otherwise the Err will be returned.
        // The difference between the operator `?` and the `match` expression is that error.
        // The function read_username can further be simplified using chains:
        fn _read_username_v2() -> Result<String, Error> {
            let mut username = String::new();

            File::open("hello.txt")?.read_to_string(&mut username);

            Ok(username)
        }
        // The creation of the string `username` didn't change, so it's moved at the beginning.
        // For reading data from the file, instead, all the operations are chained using the `?` operator.
        // The function returns a Result as well
        // Since reading a file into a string is pretty common, the standard library provides a way to open a file, create a new String, read the content and put it in the string
        // The function is std::fs::read_to_string, even if this doesn't allow to explain the error handling:
        fn _read_username_v3() -> Result<String, Error> {
            std::fs::read_to_string("hello.txt")
        }
    }
    {
        // The `?` operator can only be used in functions with compatible return type, since it prevedes an early return, similarly to match above.
        // use std::error::Error;
        use std::fs::File;
        // The folowing funciton will give an error since the return type is wrong:
        fn _read_file() {
            let _file = File::open("hello.txt");

            // _file // Wrong return type.
        }
        // There are to choices to solve this:
        // - change the return type of the function to be compatible with `?`, such as `Result<T, E>`
        // - Use `match` to handle the result
        // In any case `?` is compatible with Option too. If the value is valid, it will be inside of Some, otherwise NOne will be returned
        fn _last_char_of_first_line(text: &str) -> Option<char> {
            // Return Option because it is possible there is a char or not
            text.lines().next()?.chars().last()
        }
        // Take a text, extract the lines, get the next line, extarct teh chars and return the last one
        // If there is no first line (I.e. empty string, or first line empty) next will return None, so, with `?`, this returns Null as soon as it arrives there
        // The `chars` function returns an iterator over the chars of the string, last returns the last char of the iterator
        // The `?` operator can be used on Result and on Option, but they can't be mixed or converted. This can be done explicitly using the `ok` method of Result, or the `ok_or` of Option
        // The main function it's the netry point, so it has restrictions on the return type, however it allows to return Result<(), E>
        // For a generic error the type could be `Result<(), Box<dyn std::error::Error>>`, where `Box<...>` can be seen as "any kind of error"
        // When the main returns Result<(), E>, it returns 0 if main ends in `Ok(())`, or nonzero if it ends up in error, compatibly with C
        // The main may return any type that implements the `std::process::Termination` trait that contains a function `report` that returns an `ExitCode`
    }
}

fn when_panic() {}
