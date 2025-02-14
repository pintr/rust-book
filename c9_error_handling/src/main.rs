/// Rust has many features for error handling, additionally it requires to aknowledge the possibility of errors, so it requires to handle them.
/// There are two errors: recoverable and unrecoverable.
/// Recoverable errors are those that can be handled and the program can continue. It uses the Result<T, E> enum.
/// Unrecoverable errors are those that are not possible to handle and the program must be stopped. It uses the panic! macro.
/// 

fn main() {
    unrecoverable_errors();
}

fn unrecoverable_errors() {
    //! Unrecoverable errors are characterized by the panic! macro.
    //! The panic can be caused by the code, or by explicit call to the panic! macro.
    // When a panic occurs, Rust by default unwinds (walks back) the stack, cleans up the memory, and then exits.
    // Another possibility is to abort without unwinding the stack, which is faster, but the memory is not cleaned up. This will be done by the OS.
    // To switch to abort `panic = 'abort'` must be added to the profile in the [profile] section of the Cargo.toml file.
    // panic!("crash and burn"); // Causes a panic with the message "crash and burn".

    // When the panic occurs the program will print the line where the panic occurred, print the message, and then exit.
    let v = vec![1,2,3,4];
    v[99]; // This will cause a panic because the index is out of bounds.
    // In C or C++ this would cause a buffer overflow, which is a security vulnerability.

    // The backtrace helps to identify the cause of the panic. RUST_BACKTRACE=1 must be set in the environment variables.
    // It's enough to set the environment variable in the terminal before running the program: RUST_BACKTRACE=1 cargo run.
    // To print even more information, RUST_BACKTRACE=full can be set.
    // With `cargo build` and `cargo run` debug symbols are enabled byu default

}
