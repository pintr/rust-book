//! Building ThreadPool Using Compiler Driven Development

// Currently the `ThreadPool` type or module doesn't exist, so it needs to be built, it will be independent from the web server

use std::{
    sync::{Arc, Mutex, mpsc}, // [5] Bring into scope `Arc`, `Mutex`, and `mpsc` to create the channel, and manage the shared ownership.
    thread, // [3] Bring into scope `std::thread` since the type used is `thread::JoinHandle`
};

/// Struct that represents the ThreadPool
pub struct ThreadPool {
    // [3] Make `ThreadPool` hold a vector of `thread::JoinHandle<()>`
    // threads: Vec<thread::JoinHandle<()>>,
    // [4] Change the `ThreadPool` vector to hold `Worker` instead
    workers: Vec<Worker>,
    // [5] Add the sender of the channel created in the `ThreadPool`
    // sender: mpsc::Sender<Job>,
    // [8] TO explicitly drop the `sender` an `Option` is needed to move `sender` out of `ThreadPool` with `Option::take`
    sender: Option<mpsc::Sender<Job>>,
}

// [5] Currently the structu `Job` doesn't hold anything, but will be the type to send down the channel.
// struct Job;
// [6] `Job` must become a type alias for a trait object that holds the type of closure that `execute` receives
type Job = Box<dyn FnOnce() + Send + 'static>;

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

        // [5] Create a new channel, the pool will have the sending side, while the rokers the receiver
        let (sender, receiver) = mpsc::channel();

        // [5] It's not possible to pass `receiver` to multiple `Worker` instances, because a channel expects multiple producer, but a single consumer.
        // So the consuming side can't be cloned, additionally a message should arrive to a single `Worker`, not multiple
        // Furthermore, taking a job off the channel mutates the `receiver`, so the threads need a safe way to ahre and modify `receiver` to avoid race conditions.
        // To share ownership across multiple threads and allow the threads to mutate the value `Arc<Mutext<T>>` is used
        // The `Arc` type lets multiple `Worker` instances own the receiver
        // `Mutex` ensures that only one `Worker` gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

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
            // workers.push(Worker::new(id));
            // [5] Pass the receiver side of the channel to the worker
            // workers.push(Worker::new(id, receiver));
            // [5] For each new Worker, the `Arc` is cloned to bump the reference count so the `Worker` instances can share ownership of the receiver
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // ThreadPool // [1]
        // [3] Return a `ThreadPool` instance containing the threads.
        // ThreadPool { threads }
        // [4] Return the `ThreadPool instance containing the workers
        // ThreadPool { workers }
        // [5] Return the `ThreadPool` with workers and the sender of the channel
        // ThreadPool { workers, sender }
        // [8] The `ThreadPool` needs to return the sender in an `Option` to move the `sender` out
        ThreadPool {
            workers,
            sender: Some(sender),
        }
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
        // [6] After creating a new `Job` instance using the closure in `execute`, the job is sent down the channel.
        // `unwrap` is called on `send` for the case the sending fails, e.g. when all threads are stopped, threads can't be stopped, but the compiler doesn't know it.
        let job = Box::new(f);
        // self.sender.send(job).unwrap();
        // [8] Since sender is now an `Option` it needs to be taken as a reference using `as_ref`
        self.sender.as_ref().unwrap().send(job).unwrap();
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
    // 3. Define a `Worker::new` function that takes an `id` and returns a `Worker` instance that holds the `id` and a thread spawned with an empty closure.
    // 4. In `ThreadPool::new`, use the for loop counter to generate the `id`, create a new `Worker` with that `id`, and store the worker in the vector.
    // The code in `src/main.rs` doesn’t need to know the implementation details regarding using a `Worker` struct within `ThreadPool`
    // So the `Worker` struct and its `new` function can be private
    // Note that if there aren't enough system resources, `thread::spawn` will panic
    // In a production thread pool implementation the method `spawn` of a ` std::thread::Builder` would be used, because it return a `Result`
    // Now the closure needs to be processed.
    // [5] Sending Requests to Threads via Channels
    // Currently the closures given to `thread::spawn` do nothing, instead the closure passed to `execute` should be run.
    // The `Worker` should fetch the closure from a queue in the `ThreadPool`, and send it to its thread
    // A way to do it is using a channel to send the job form `ThreadPool` to the `WOrker` instances:
    // 1. The `ThreadPool` will create a channel and hold on to the sender.
    // 2. Each `Worker` will hold on to the receiver
    // 3. A `Job` struct will be created to hold the closures to send down the channel.
    // 4. THe `execute` method will send the job to be executed through the sender.
    // 5. In its thread, the `Worker`will loop over its receiver, and execute the closures of any jobs it receives.
    // [6] Implementing the execute Method
    // Finally, the `execute` method on `ThreadPool` can be implemented

    // Graceful Shutdown and Cleanup
    // Currently, the code works properly, but there are some warnings signalling that `workers`, `id`, and `thread` fields are not used directly, meaning nothing is cleaned up.
    // Using ctrl-c the main thread is halted, interrupting all the other threads as well.
    // The next steps are implementing the `Drop` trait to call `join` on each of the threads in the pool, so they can close the request they are working on before closing.
    // Another freature to implement is a way to tell the  threads they should stop accepting new requests and shut down
    // [7] Implementing the Drop Trait on ThreadPool
    // [8] Signaling to the Threads to Stop Listening for Jobs
    // Now the code compiles without warnings, but the behaviour is not the one desired because of the logic in the closures run by the threads of the `Worker` instances.
    // Currently, calling `join` won't shut down the threads because they `loop` forever looking for jobs, so the main thread would block forever, waiting for the first thread to finish.
    // To fix this the `ThreadPool drop`, and `Worker` loop need to be changed
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // [8] Drop the sender to close the channel, so no more messages will be sent.
        // Now all the calls to `recv` that the `Worker` instances do infinitely will return an error.
        drop(self.sender.take());
        // [7] When the pool is dropped, the threads whould all join making sure they finish their work
        // The loop goes though each `worker` in the thread pool, `&mut` is used since `self` is a mutable reference, and `worker` needs to mutate too.
        // With this notation the compiler gives an error saying that `join` can't be called because there is only a jmutable borrow of each worker, and `join` takes ownership of its argument.
        // To solve this issue the thread needs to be moved out the `WOrker` instance that owns `thread` so `join` can consume the thread.
        // A solution could be using `Option` in order to use `take` to move the value out of `Some` while leaving a `None`, but this would be useful only for dropping, while dealing with `Option` for each other operation.
        // for worker in &mut self.workers {
        // [7] // A better alternative is using `Vec::drain`, which accepts a range parameter to specify which items to remove, and returns an iterator on those items. With `..` it would be every value
        for worker in &mut self.workers.drain(..) {
            // [7] For each worker a message is printed saying that the particular `Worker` is shutting down
            // Then `join` is used to that particular worker, with `unwrap` in case `join` fails, so Rust will panic.
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    // [4] Create the `Worker` struct that has an `id` to distinguish between different instances of `Worker`
    id: usize,
    // [4] The `Worker` struct holds a single `thread::JoinHandle<()>`
    // Later `Worker` will have a method to take a closure of code and send it to the already running thread for execution
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // [4] The `new` spawns a thread with an empty closure and stores it in `thread`
        // [5] Pass the receiver side of the channel to the Worker instances, so the `receiver` parameter can be referenced in the closure.
        // The signature needs to be `receiver: Arc<Mutex<mpsc::Receiver<Job>>>` instead of `receiver: mpsc::Receiver<Job>` because the receiver side of the channel is shared between multiple workers
        // let thread = thread::spawn(|| {
        //     receiver;
        // });

        // [6] In the previous version, the closure being passed to `thread::spawn` only references the receiving end of the channel.
        // The closure should loop forever, asking the receiving end for a job, and run it when there is one.
        let thread = thread::spawn(move || {
            loop {
                // [6] At first the `lock` on `receiver` is called to acquire the mutes, then `unwrap` is called to panic on errors.
                // The lock might fail if the mutes is in a poisoned state: a thread panicked while holding the lock.
                // If the lock is acquired, the `recv` method is used to get the `Job`, which is unwrapped to move past any errors, which might occur if the sender has shut down.
                // The call to `recv` blocks, so, if there is no job yet, the thread will wait until a job becomes available.
                // Only one `Worker` thread at time is trying to request a job because of the `Mutex<T>`.
                // let job = receiver.lock().unwrap().recv().unwrap();

                // println!("Worker {id} got a job; executing.");

                // job();
                // [8] Dropping `sender` closes the channel, so no more mesages can be sent, so all the calls to `recv` will returnan error
                // The loop is changed to gracefully exit the loop in that case, so the threads will finish when `THreadPool drop` calls `join` on them.
                // The main needs to be changed to test this, limiting the number of requests before shutting down the server.
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        // [4] The `Worker` is created and returned with the passed `id` and `thread`
        Worker { id, thread }
    }
}
