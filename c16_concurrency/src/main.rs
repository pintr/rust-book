//! Rust safely and efficiently handles concurrent programming (parts of the program execute independently) and parallel programming (parts of the program execute at the same time)
//! Ownership and type systems are a powerful set of tools to manage memory safety and concurrency problems, resulting in compile time errors rather then runtime errors.
//! This aspect of Rust is called fearless concurrency and allows to write code without bugs and easy to refactor, while offering a variety of tools for doing so.
//! In particular Rust offers threads creation and handling, message-passing concurrency, shared state concurrency, and `Sync` and `Send` traits to extend concurrency guarantees

use std::{
    // rc::Rc,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    threads();
    message_passing();
    shared_state();
    send_sync_trait();
}

fn threads() {
    // In most OSs a program is run in a process, within a process there can be independent parts that run simultaneously called threads
    // Splitting the computation into multiple threads to run multiple tasks can improve performances, but adds complexity.
    // Since threads run simultaneously there is no guarantee about the order in which parts of the code are executed, leading to problems such as:
    // - Race conditions: threads access data in an inconsistent order
    // - Deadlocks: two threads wait for each other preventing both from continuing
    // - Bugs hard to reproduce and fix
    // Rust uses a 1:1 model where a program uses a OS thread for each language thread
    // The function `thread:spawn` is used to create a new thread, it accepts a closure as parameter containing the code to run:
    {
        thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} form the spawned thread!"); // Print number
                thread::sleep(Duration::from_millis(1)); // Stop execution for 1ms
            }
        });

        for i in 1..5 {
            println!("hi number {i} form the main thread!"); // Print number
            thread::sleep(Duration::from_millis(1)); // Stop execution for 1ms
        }
        println!();
    }
    // When the main thread of a Rust program completes, all spawned threads are shut down whether or not they have finished running.
    // In this case the execution is pausd 1ms, allowing both threads to run.
    // The threads will probabliy take turns, but it's not guarantee since it is managed by the OS
    // In this case most probabliy the spawned thread isn't able to complete befor the main thread shuts down.
    // To prevent the spawned thread to stop prematurely, and guarantee it is run
    // It is possible to save the return value of the thread in a variable, preventing from ending prematurely
    // The return type of `thread::spawn` is `JoinHandle<T>`, which is an owned value that has the `join` method that allows to wait for the thread to finish.
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} form the spawned thread!"); // Print number
                thread::sleep(Duration::from_millis(1)); // Stop execution for 1ms
            }
        });

        // handle.join().unwrap(); // The main thread will wait the spawned thread to finish before executing the following code
        for i in 1..5 {
            println!("hi number {i} form the main thread!"); // Print number
            thread::sleep(Duration::from_millis(1)); // Stop execution for 1ms
        }

        handle.join().unwrap(); // Make sure all the threads finish, they alternate until main finishes
    }
    // The `join` method blocks the main thread from performing work or exiting
    // In this case the two threads continue alternating, bu the main waits for the spanw to finish thanks to `handle.join()`
    // Often the `move` keyword is used passed to `thread::spawn` because the closure will take ownership of the values
    // In the previous examples the spawned thread doesn't use any data from the main thread, to use data from the main thread, the spawned thread must capture the values it needs.
    {
        // let v = vec![1, 2, 3];

        // Rust infers how to capture `v` and, because it only needs a reference, it borrows it but Rust can't tell how long the spawned thread will run, so it doesn't know if the reference to v will always be valid
        // let handle = thread::spawn(|| {
        //     println!("Here's a vector: {v:?}");
        // });

        // drop(v);
        // If Rust allowed to run this code the spawned thread wouldn't probably start before `v` is dropped, even if it has a reference to it, so it's no longer valid

        // handle.join().unwrap();
    }
    {
        // By adding the `move` keyword before the closure, the closure is forced to take ownership of the values it uses instead of allowing Rust to infer that it should borrow
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        // drop(v); // This doesn't work because the main thread no longer owns `v` since it has been moved to the closure of the spawned thread.

        handle.join().unwrap();

        // By telling Rust to move ownership of `v` to the spawned thread, it's guaranteed that the main thread won't use `v` anymore, overriding the Rust's conservative default of borrowing.
    }
}

fn message_passing() {
    // One way to ensure safe concurrency is message passing, where threads communicate by sending each other messages containing data.
    // Rust provides an implmentation of channels, which is a communication method between a transmitter, that sends the messages, and a receiver, who receives them. A channel is closed when one of the two is dropped.
    {
        // A channle is created using `mpsc::channel`, where `mpsc` stands for Multiple Producer, Single Consumer meaning that Rust allow multiple sending ends, but only a receiver
        // The `mpsc::channel` returns a tuple where the first element `tx` is the sending end, while the second `rx` is the receiving end.
        // The tuple is destructured using a `let` statement
        let (tx, rx) = mpsc::channel(); // Won't compile alone because the types of values are not specified

        thread::spawn(move || {
            let val = String::from("hi");
            println!("Send: {val}");
            tx.send(val).unwrap(); // `tx` is moved in the closure, so it is owned by it.
                                   // The `send` method returns a `Result<T, E>` so, if the receiver has been dropped,the send operation will return an error, in this case it panics in case of an error.

            // println!("Sent: {val}"); // This gives an error because the value has been sent to another thread, and that thread could modify or drop it, causing errors,
            // The `send` function takes ownership, and the receier takes ownership when the value is moved.
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
        // The receier has two useful methods:
        // - `recv` will block the main trhread execution and wait until a value is present in the channel, it will return `Result<T, E>`, when the transmitter closes `recv` will return an error.
        // - `try_recv` does not block the main thread, but returns a `Result<T, E>` immediatly, `Ok` if it holds a value, `Err` otherwise
    }
    {
        // In this example the thread sends multiple messages with a pause of 1 second between each of them
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }

        // `rx` can be treated as an iterator and, each value received is printed until the channel is closed, waiting for the messages
    }
    {
        // Since `mpsc` stands for multiple producers, single consumer, it is possible to send values from multiple threads.
        // This can be done by cloning the transmitter, resulting in a new transmitter to `rx`, used by two different threads.
        let (tx, rx) = mpsc::channel();
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}")
        }
    }

    // The order may vary depending on the system, for this reason concurrency is difficult: it's nondeterministic.
}

fn shared_state() {
    // Shared memory is another method of sharing data with the difference that allows multiple ownership: multiple threads can access the same memory location at the same time
    // Multiple ownership can add complexity, for this reason Rust type system and ownership rules assist in the correct management
    // A useful tool is the "mutex", which stands for "mutual exclusion", as it allows only one thread to acces data in any give time.
    // To access it a thread first signals that it wants to access by asking to acquire the mutex lock: data structure that keeps track who has exclusive access to the data.
    // The mutex guards the data and, in order to access it there are two rules: attempt to acquire the lock before using the data, and unlock the data when finished, so other can access it
    let m = Mutex::new(5); // Mutex created with data = 5, the type of `m` is `Mutex<i32>` so the `lock` method must be called before being able to use the `i32` value

    {
        let mut num = m.lock().unwrap(); // Acquire the lock, this will block the current thread so it stops until the lock is obtained.
                                         // The lock fails when another thread holding the lock panics, in that case no one would be able to get it, with `unwrap` the thread panics if this happens
                                         // `Mutex<T>` is a smart pointer, the call to `lock` returns a smart pointer `MutexGuard`, wrapped in `LockResult` handled calling `unwrap`.
                                         // `MutexGuard implments `Deref` to point innner data, and also has a `Drop` implementation that releases the lock when it goes out of scope.
        *num = 6; // Once acquired the lock the return value is a mutable reference to the data inside
    }

    println!("m = {m:?}");

    // Example of sharing a `Mutex<T>` between multiple threads
    {
        // let counter = Mutex::new(0);
        // let mut handles = vec![];

        // for _ in 0..10 {
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }

        // for handle in handles {
        //     handle.join().unwrap();
        // }

        // println!("Result: {}", counter.lock().unwrap()); // Doesn't work because the counter value was moved in the previous iteration of the loop.
        // The ownership of the lock `counter` can't be moved into multiple threads.
    }
    {
        // let counter = Rc::new(Mutex::new(0));
        // let mut handles = vec![];

        // for _ in 0..10 {
        //     let counter = Rc::clone(&counter);
        //     let handle = thread::spawn(move || {
        //         let mut num = counter.lock().unwrap();
        //         *num += 1;
        //     });
        //     handles.push(handle);
        // }

        // for handle in handles {
        //     handle.join().unwrap();
        // }

        // println!("Result: {}", *counter.lock().unwrap());
        // in this case the error is `Rc<Mutex<i32>>` cannot be sent between threads safely`
        // This because `Rc<Mutex<i32>>` requires the trait `Send`, required for concurrency.
        // `Rc<T>` is not safe to share across threads: it manages the reference count, but it's not able to make sure that changes to the count can't be interrupted by another thread
    }
    {
        // `Arc<T>` is a type like `Rc<T>` but it is safe to use in concurrency situations. They share the API too.
        // Arc stands for atomically reference-counted, so it is an atomic type, which is safe for share
        // Thread safety introduced performance penality, for this reason not all primitive types are atomic.
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
        // `counter` is immutable but it is possible to get a mutable reference to the value inside, so a `Mutex<T>` provides interior mutability as the `Cell` familiy does.
        // As teh same way a `RefCell<T>` is used to mutate the content inside of a `Rc<T>`, the `Mutex<T>` is used to mutate the content inside `Arc<T>`
        // As `Rc<T>` comes with the risk of reference cycles, similarly `Mutex<T>` comes to the risks of deadlocks.
        // Deadlocks happen when an operation needs to lock two resources, and two threads have acquired each of one, causing them to wait forever.
    }
}

fn send_sync_trait() {
    // The `send` marker trait indicates that the ownership of values of type implementing `Send` can be transferred beteen threads.
    // Almost every Rust type is `Send` but there are some exception such as `Rc<T>` because multiple threads could update the reference count at the same time
    // For concurrency `Arc<T>` substitutes `Rc<T>` affecting the performances
    // The `Sync` marker trait makes it safe to be referenced from multiple threads. Any type `T` implements  `Sync` if `&T` implements `Send`.
    // All primitive types implement `Sync`, smart pointers `Rc<T>` and `RefCell<T>` don't. The smart pointer `Mutex<T>` implements it and can be accessed by multiple threads.
    // Types composed entirely by other types implmenting `Send` and `Sync` automatically implement it, so they don't need to be implmented manually.
    // Manually implementing these traits involves implementing unsafe Rust code.
}
