//! There are multiple techniques to perform operation while waitin other long running processes to complete: parallelism, concurrency, and asynchronous programming
//! In asynchronous programming operations may not finish sequentially in the order they were started.
//! Rust provides futures, streams, the `async` and `await` syntax, and other tools for managing and coordinating asynchronous operations.
//! OSs already provide a form of concurrency with interrupts, which allow to switch context during its use, allowing multiple process to run simultaneously.
//! In the same way programs may need to perform many operations in the same time, such as multiple downloads while using the UI: everything should work simultaneously without interruptions.
//! For example a software downloading files could spawn a dedicated thread for each file to be downloaded without blocking the main thread.
//! Rust allows to write asynchronous code in the same style of blocking code using `async` and `await` to solve concurrency problems, assitionally it allows to combine the async code with concurrency.
//! There is difference between concurrency and parallelism:
//! - Concurrency: an individual works on several different task before any of them is completed, requires switching.
//! - Parallelism: two different task work independently at the same time.
//! In both workflows it may be necessary to coordinate the defferent tasks. Some work may be done in parallel, other could be serial (one after the other)
//! For example  amachine with a single CPU core can perform an operation at a time, but it can work concurrently by switching its context. With multi-core it can work in parallel too.
//! In Rust with `async` it's dealing cocnurrency, but it may use parallelism too.

fn main() {
    println!("Hello, world!");
}
