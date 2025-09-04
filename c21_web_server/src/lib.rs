//! Building ThreadPool Using Compiler Driven Development

// Currently the `ThreadPool` type or module doesn't exist, so it needs to be built, it will be independent from the web server

/// Struct that represents the ThreadPool
pub struct ThreadPool;

// Now that the `ThreadPool` struct has been craeted, the compiler tells to create an associated function called `new`
// The `new` function accepts an integer argument that represents the number of threads
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // `usize` is chosen as the type of the parameter `size` because a negative number wouldn't make sense
        // This is the simplest version of `new` updates will come.
        ThreadPool
    }
    // After creating the `new` method, the compiler tells that the `execute` method on `ThreadPool` is missing
    // `execute` should have a similar interface to `thread::spawn`, and it takes a closure that is given to an idle thread in the pool
    // The signature of `thread::spawn` is the following:
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // THe `F` type parameter is the once that concern the the web server, the `T` type is used related to the return value, that doesn't interest the server.
    // `spawn` uses the `FnOnce` as the trait bound to `F`, which is the same to use for the thread pool, since it will be passed from `execute` to `spawn`.
    // Additionally the thread running a request will only execute that closure one time, mathcing the `Once` in `FnOnce`.
    // The `F` type parameter alsa has the trait bound `Send` and the lifetime bound `'static` which are useful for the server:
    // `Send` is used to transfer the closure from one thread to another, `'static` because it's not known how long the thread will take to execute
    // Here is the implementation of the `execute` method:
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // `()` is used after `FnOnce` because it represents a closure that takes no parameters, and returns the unit type `()`, the return type can be omitted from the signature
    {
    }
    // Now the code compiles, but it gives error in the browser, since the library isn't calling the closure passed to `execute` yet.
}
