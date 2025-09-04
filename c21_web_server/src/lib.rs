//! Building ThreadPool Using Compiler Driven Development

// Currently the `ThreadPool` type or module doesn't exist, so it needs to be built, it will be independent from the web server

use std::thread; // [3] Bing into scope `std::thread` since the type used is `thread::JoinHandle`

/// Struct that represents the ThreadPool
pub struct ThreadPool {
    // [3] Make `ThreadPool` hold a vector of `thread::JoinHandle<()>`
    // threads: Vec<thread::JoinHandle<()>>,
    // [4] Change the `ThreadPool` vector to hold `Worker` instead
    workers: Vec<Worker>,
}

// Now that the `ThreadPool` struct has been craeted, the compiler tells to create an associated function called `new`
// The `new` function accepts an integer argument that represents the number of threads
impl ThreadPool {
    // [1] first version
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        // [1] `usize` is chosen as the type of the parameter `size` because a negative number wouldn't make sense

        // [2] Since a pool with 0 threads doesn't make any sense but it's valid, check that `size` is greater than 0
        // Additionally, the documentation has been added using doc comments, can be opened using `cargo doc --open`
        // Instead of adding the `assert!` macro, `new` could have been changed into `build` asn return a `Result`, but creating a pool with 0 threads is an unrecoverable error.
        // The `build` signature would have been: `pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>`
        assert!(size > 0);

        // [3] Once a valid size is received, `ThreadPool` creates a new vector the can hold `size` items
        // THe `with_capacity` function it's as a `new`, but pre-allocates space in the vector, since the size is known
        // This way is slightly more efficient, because `new` resizes itself as elements are inserted.
        // let mut threads = Vec::with_capacity(size);
        // [4] change threads to workers
        let mut workers = Vec::with_capacity(size);

        // [3] Set up a loop that will create the threads.
        // for _ in 0..size {
        // [4] The `for` loop changes using the index as `id` for the workers
        for id in 0..size {
            // [3] TODO: create some threads and store them in the vector.
            // [4] Add the workers to the vector.
            workers.push(Worker::new(id));
        }

        // ThreadPool // [1]
        // [3] Return a `ThreadPool` instance containing the threads.
        // ThreadPool { threads }
        // [4] Return the `ThreadPool instance containing the workers
        ThreadPool { workers }
    }
    // After creating the `new` method, the compiler tells that the `execute` method on `ThreadPool` is missing
    // `execute` should have a similar interface to `thread::spawn`, and it takes a closure that is given to an idle thread in the pool
    // The signature of `thread::spawn` is the following:
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T,
    //     F: Send + 'static,
    //     T: Send + 'static,
    // The `F` type parameter is the once that concern the the web server, the `T` type is used related to the return value, that doesn't interest the server.
    // `spawn` uses the `FnOnce` as the trait bound to `F`, which is the same to use for the thread pool, since it will be passed from `execute` to `spawn`.
    // Additionally the thread running a request will only execute that closure one time, mathcing the `Once` in `FnOnce`.
    // The `F` type parameter alsa has the trait bound `Send` and the lifetime bound `'static` which are useful for the server:
    // `Send` is used to transfer the closure from one thread to another, `'static` because it's not known how long the thread will take to execute
    // Here is the implementation of the `execute` method [1]:
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static, // `()` is used after `FnOnce` because it represents a closure that takes no parameters, and returns the unit type `()`, the return type can be omitted from the signature
    {
    }
    // Now the code compiles, but it gives error in the browser, since the library isn't calling the closure passed to `execute` yet.
    // [2] Validating the Number of Threads in new
    // Currently the parameters of `new` and `execute` aren't doing anything
    // [3] Creating Space to Store the Threads
    // Now that a valid number is granted it is necessary to create and store the threads in the `ThreadPool` before returning the struct
    // To store a thread, as in `thread::spawn` the used funtion `JoinHandle`, in particular `JoinHandle<T>` where `T` is the type the closure returns
    // In this case the closures passed to the thread don't return anything, so `T` will be the unit type `()`
    // [4] A Worker Struct Responsible for Sending Code from the ThreadPool to a Thread
    // In the previous stepa comment was left regarding the creation of threads, in this step the threads will be created
    // THe standard library provides `thread::spawn` to create a thread, but it expects to get the code to run when it's created
    // In this case the threads need to be created and then wait for code to run when sent later
    // This behaviour doesn't exist in the standard library, so it needs to be created using a data structure between `ThreadPool` and threads, named `Worker`
    // The `Worker` picks code to run and runs the code in the Worker's thread.
    // Here is the process that happens when a `ThreadPool` is created:
    // 1. Define a `Worker` struct that holds an `id` and `JoinHandle<()>`.
    // 2. Change `ThreadPool` to hold a vector of `WOrker` instances.
    // 3. Define a `WOrker::new` function that takes an `id` and returns a `Worker` instance that holds the `id` and a thread spawned with an empty closure.
    // 4. In `ThreadPool::new`, use the for loop counter to generate the `id`, create a new `Worker` with that `id`, and store the worker in the vector.
    // The code in `src/main.rs` doesn’t need to know the implementation details regarding using a `Worker` struct within `ThreadPool`
    // So the `Worker` struct and its `new` function can be private
    // Note that if there aren't enough system resources, `thread::spawn` will panic
    // In a production thread pool implementation the method `spawn` of a ` std::thread::Builder` would be used, because it return a `Result`
    // Now the closure needs to be processed.
    // [5] Sending Requests to Threads via Channels
}

struct Worker {
    // [4] Create the `Worker` struct that has an `id` to distinguish between different instances of `Worker`
    id: usize,
    // [4] The `Worker` struct holds a single `thread::JoinHandle<()>`
    // Later `Worker` will have a method to take a closure of code and send it to the already running thread for execution
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        // [4] The `new` spawns a thread with an empty closure and stores it in `thread`
        let thread = thread::spawn(|| {});
        // [4] THe `Worker` is created and returned with the passed `id` and `thread`
        Worker { id, thread }
    }
}
