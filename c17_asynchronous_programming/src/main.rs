//! There are multiple techniques to perform operation while waiting other long running processes to complete: parallelism, concurrency, and asynchronous programming
//! In asynchronous programming operations may not finish sequentially in the order they were started.
//! Rust provides futures, streams, the `async` and `await` syntax, and other tools for managing and coordinating asynchronous operations.
//! OSs already provide a form of concurrency with interrupts, which allow to switch context during its use, allowing multiple process to run simultaneously.
//! In the same way programs may need to perform many operations in the same time, such as multiple downloads while using the UI: everything should work simultaneously without interruptions.
//! For example a software downloading files could spawn a dedicated thread for each file to be downloaded without blocking the main thread.
//! Rust allows to write asynchronous code in the same style of blocking code using `async` and `await` to solve concurrency problems, additionally it allows to combine the async code with concurrency.
//! There is difference between concurrency and parallelism:
//! - Concurrency: an individual works on several different task before any of them is completed, requires switching.
//! - Parallelism: two different task work independently at the same time.
//! In both workflows it may be necessary to coordinate the defferent tasks. Some work may be done in parallel, other could be serial (one after the other)
//! For example a machine with a single CPU core can perform an operation at a time, but it can work concurrently by switching its context. With multi-core it can work in parallel too.
//! In Rust with `async` it's dealing concurrency, but it may use parallelism under the hood.

use std::future::Future;

fn main() {
    futures_async();
    concurrency_with_async();
    multiple_futures();
    streams();
    traits_async();
    futures_tasks_threads();
}

fn futures_async() {
    // The main elements of async programming in Rust are futures, and `async/await` keywords
    // A `future` is a value that may be not ready now, but it will in the future.
    // Rust provides a `Future` trait as a building block, so async operations can be imlemented with a common interface
    // In Rust, `futures` are types that implement the `Future` trait, and each future holds its own information on the progress made, and when it's ready.
    // Thye `async` keyword can be applied to code and functions so they can be interrupted and resumed.
    // In an `async` block or function, `await` is used to await a future to be ready. The process of checking the state of the future is called `polling`
    // When using `async/await`, Rust compiles into code using the `FUture` trait, which compiles `for` loops into iterators. Custom types can be implemented using the trait `Future`.
    {
        // An example is a little web scraper:
        // The crate `trpl` has been created to re-export all the needed features (e.g. `futures`, `tokio`)
        // The `futures` crate is official from Rust and the `FUture` trait was originally designed there
        // The `tokio` crate is the most widely used async runtime in Rust

        use trpl::{Either, Html};

        /// Async function that extracts the tiltle of a web page
        ///
        /// First of all it performs a GET to obtain the web page and awaits the response
        /// Once the response is available the whole text is awaited and extracted
        /// The await keyword needs to be explicitly asked,since Rust futures are lazy: they don't anything until asked
        /// Once the response_text is available, it can be parsed in an instance of `Html` type
        /// The `Html` type allows to navigate and query the DOM
        /// The `select_first` method returns an `Option<ElementRef>` containing the first elmement requested (in this case `title`) if it exists
        /// Then the `Option::map` method, similarly to |match|, is used to work with the item in the option
        /// In the body of the map `inner_html` is called to get the content of `title_element`
        /// The result is an `Option<String>` containing the page title (if it doesn't exist `None`)
        async fn page_title(url: &str) -> Option<String> {
            // let response = trpl::get(url).await;
            // let response_text = response.text().await;
            // The keyword `await` goes after the expression to make the chains of method nicer to work with
            let response_text = trpl::get(url).await.text().await;
            Html::parse(&response_text)
                .select_first("title")
                .map(|title_element| title_element.inner_html())
        }
        // When Rust sees a block with the `async` keyword, it compiles it into a unique, anonymous data type that implements the `Future` trait
        // When RUst sees a function marked with `async` it compiles it into a non-async function whose body is an async block, the return type is the type of the anonymous data type
        {
            // The above function can be equivalently written as follows:
            // It uses the `impl Trait` syntax and the returned trait is a Future with an associated type of Output
            fn _page_title(url: &str) -> impl Future<Output = Option<String>> {
                // Since the function parameter `url` is a borrowed string slice, it doesn't own the data it points to, so the borrowed value needs to be converted to an owned `String`
                let url = url.to_string();
                // All the code is in a`sync move` block, which is an expression returned from the function.
                // The async block produces a value of type `Option<String>` which matches the `Output` type of the return type
                // The body is in a n `async move` block because it uses the `url` parameter
                async move {
                    let text = trpl::get(&url).await.text().await;
                    Html::parse(&text)
                        .select_first("title")
                        .map(|title| title.inner_html())
                }
            }
        }
        // This function couldn't be used directly in the main, because it would need to be async, which is not allowed because `async` requires a runtime (Rust crate that manages the execution of asynchronous code), and the `main` function is not a runtime, but it can initialise one
        // Most languages that support async bundle a runtime, Rust doesn't, instead it has different async runtimes available used in different cases (such as multiple CPU corse compare to a single one)
        // To initialise a runtime there is the function `trpl::run` that runs a single future to completion on a Tokio Runtime. Once the future completes, `run` returns the value produced
        // The future returned by `page_title` can be passed to run and, once completed, the result can be extracted using `match`
        // For most async code, there are more then one async function calls, so instead of a single function, a whole async block is passed.
        trpl::run(async {
            let url = "https://www.rust-lang.org";

            match page_title(url).await {
                Some(title) => println!("The title of {url} is {title}"),
                None => println!("{url} has no title"),
            }
        });
        // Each `await` point, so where `await` is used, represents a place where control is handed back to the runtime.
        // Rust needs to keep track of the state invoked in the async bock so the runtime can go on with other stuff and come back when it's ready to try to advance the first one.
        // This invisible state machine is represented as an enum used to save the current state at each await point
        enum _PageTitleFuture<'a> {
            Initial { url: &'a str },
            GetAwaitPoint { url: &'a str },
            TextAwaitPoint { response: trpl::Response },
        }
        // The Rust compiler creates and manages the state machine for `async` automatically following the standard rules for borrowing and ownership. This state machine is executed by the runtime using the `executors`
        // If `main` was async something else would need to manage the state machine, but `main` is the entry point of the program, for this reason `trpl::run` is called to setup the runtime in main
        // Let's update the page_title function in order to return the URL too, this will be used to race between two async calls:
        async fn url_page_title(url: &str) -> (&str, Option<String>) {
            let text = trpl::get(url).await.text().await;
            let title = Html::parse(&text)
                .select_first("title")
                .map(|title| title.inner_html());
            (url, title)
        }

        trpl::run(async {
            // Call `url_page_title` for both the urls
            let url_title_1 = url_page_title("https://www.rust-lang.org");
            let url_title_2 = url_page_title("https://doc.rust-lang.org/");

            // The two elements are passed to `trpl::race` which returns a value to indicate which future finished first
            // The `race` function is built on a `select`, more used in the real world
            let (url, title) = match trpl::race(url_title_1, url_title_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };
            // The `Either` type returned by the `trpl:race` function is similar to a `Result` because it has two cases, but there is no notion of success, instead it indicates one or another such as:
            enum _Either<A, B> {
                Left(A),
                Right(B),
            }
            // The `race` function returns `Left` if `url_title_1` finishes first, otherwise `RIght` if `url_title_2` finishes first. It matches the order of the arguments.
            println!("{url} returned first");

            match title {
                Some(title) => println!("Its title is: '{title}'"),
                None => println!("It has no title",),
            }
        })
    }
}

fn concurrency_with_async() {
    use std::time::Duration;

    // In many cases the APIs for working with concurrency using async are very similar to those for using threads, in other cases they are similar but with a different behaviour
    // One example is spawning a new thread and make it sleep
    // Set runtime
    trpl::run(async {
        {
            // Put a loop with a pause in a new thread, assign to `handle` variable to use a join on it alter
            let handle = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            // Put a loop with a sleep in the main thread
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
            // This code behaves similarly to the thread based implementation, and the spawned thread stops as soon as the main thread finishes
            // In order to let it finish there is the need for a join handle, in case of  async the corresponding keyword is `await`, because `handle` is a future
            handle.await.unwrap();
            // Async and threads give the same result but the biggest difference is that there is no need to spawn another OS thread.
        }
        {
            // Async blocks compile in anonymous futures, so the runtime can run them both to completion using `trpl::join`
            // The `trpl::join` funciton is similar to `std::thread::spawn` but for futures.Giving it two futures it produces a single new future whose output is a tuple containing each future passed, and waits both to complete

            let fut1 = async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let fut2 = async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            trpl::join(fut1, fut2).await;

            // In this case the same exact order is kept every run, unlike with threads, this is because `trpl::join` is fair and checks every feature eqaully often, so it never lets one race ahead if the other is ready.
            // With threads is the OS that decides which thread to check, with async Rust is the runtime to do so.
            // Runtimes don't have to guarantee fairness for any operation, it depends on the selected runtime
        }
        {
            // Async allows to share messages between futures, similarly to the transfer of data between threads
            let (tx, mut rx) = trpl::channel();

            let val = String::from("hi");
            tx.send(val).unwrap();

            let received = rx.recv().await.unwrap();
            println!("Got {received}");

            // Here `trpl::channel` si used: an async version of the multiple-producer, single-consumer channel API used with threads
            // The difference is that it uses a mutable receiver `rx`, instead of an immutable one, and the `recv` method produces a future that needs to be awaited, rather than producing the value directly.
            // now messages can be sent from the ender to the receiver without spawning separated threads or task, it's just needed to await the `rx.recv` call
            // The asynchronous `Receiver::new` method in `std::mpsc::channel` blocks until it receives a message, while `trpl::Receiver::recv` does not because it's async, so it hands back the control to runtime until a message arrives, or the send side closes.
            // The send call is not awaited because it is not blocking and the channel on which is sent is unbounded.
            // `trpl::run` lets the user choose where to block on some set of async code, so where to transition from sync to async
            // In this example the message will arrive right away and, even if there is a future, there's no concurrency, everything happens in sequence.
        }
        {
            // Let's try sending more messages
            // In the following case it would be sufficient to call `rx.recv().await` four times, but in the real world the numer of messages would be unknown, so the need is to keep waiting.
            // trpl::run(async {
            //     let (tx, mut rx) = trpl::channel();

            //     let vals = vec![
            //         String::from("hi"),
            //         String::from("from"),
            //         String::from("the"),
            //         String::from("future"),
            //     ];

            //     for val in vals {
            //         tx.send(val).unwrap();
            //         trpl::sleep(Duration::from_millis(500)).await;
            //     }

            //     // For threads there is the possibility to use a `for` loop to process all the items, for async it doesnt exist, so there is the nedd of the `while let` conditional loop
            //     // This loop continues as long as the pattern it specifies continues to match the value
            //     // The `rx.recv()` call produces a future that is awaited, the runtime will pause the future until ready, once the message arrives it will resolve to `Some(message)` that can be used in the body of the loop, if the result is `None` the loop stops
            //     while let Some(value) = rx.recv().await {
            //         println!("received '{value}'");
            //     }
            // });
            // The code now sends and receives all the messages but, unfortunately, there are two problems:
            // - The messages arrive all at once, 2 seconds after the the block is launched.
            // - The program never stops.
            // The messages arrive all at once after the full delay because the order in which `await` appear ina an async block is also the one in which they are executed.
            // There is only one async block, so everything runs linearly: all the `tx.send` happen interspersed with all the `thread.sleep` call and their await points, so the `while let` get to go through any of the `await` points on the `recv` calls
        }
        {
            // To get the expected behaviour, where the sleep delays happens between each message, `tx` and `rx` nmeed to be in thier own async blocks, then the runtime will execute them separately using `trpl::join`
            // There is the need to await `trpl::join`, not the individual futures, otherwise it returns ina sequential flow.

            let (tx, mut rx) = trpl::channel();

            // let tx_fut = async { // Doesn't stop
            let tx_fut = async move {
                // Stops
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            trpl::join(tx_fut, rx_fut).await;

            // In this updated example the messages are printed at 500ms intervals rather then all togheter after 2s
            // The program still never exits because of the way `while let` interacts with `trpl::join`:
            // - The future returned from `trpl::join` completes only when both futures have completed
            // - The `tx` future completes once it finishes sleeping after sending all the messages, rx` won't complete auntil `while let` ends
            // - The `while let` loop won't end until awaiting `rx.recv` produces `None`, that happens only when the other end of the channel is closed via `rx.close`
            // - `rx.close` isn't called anywhere, and `tx` won't be dropped until the outermost async block passed to `trpl::run` ends
            // - The block can't end because it is blocked in `trpl::join`, which brings to the top of the list
            // `rx` could be closed manually calling `rx.close`  somewhere, but it doens' make much sense because it could miss messages, instead `tx` should be dropped before the end of the function
            // This could be done by moving `tx` into the async block so it would be dropped once it ends, this can be done using the `move` keyword, similarly as with threads
        }
        {
            // In this case the async channel is also a multi-producer channel, so a `clone` can be called on `tx` in order to send messages from multiple futures

            let (tx, mut rx) = trpl::channel();

            // Clone `tx` creating `tx1` outside of the `async` block (order of `tx` and `tx1` is not relevant)
            // Both need to be in an `async move` block to let the program stop
            let tx1 = tx.clone();

            let tx1_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    // Move `tx1` inside the block
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let tx_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    // Move the original `tx` into a new async block with different characteristics
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            // join the three futures
            trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        }
    });
}

fn multiple_futures() {
    use std::{
        pin::{Pin, pin},
        thread,
        time::{Duration, Instant},
    };
    use trpl::Either;
    trpl::run(async {
        {
            let (tx, mut rx) = trpl::channel();

            // Clone `tx` creating `tx1` outside of the `async` block (order of `tx` and `tx1` is not relevant)
            // Both need to be in an `async move` block to let the program stop
            let tx1 = tx.clone();

            let tx1_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    // Move `tx1` inside the block
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            let tx_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    // Move the original `tx` into a new async block with different characteristics
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            // In order to join three futures the `join` function was used, while for three of them, the `join3` was used, but it's annoying changing the function each time
            // trpl::join3(tx1_fut, tx_fut, rx_fut).await;
            // For this reason there is the macro form of `join` that accepts an aribitrary number of arguments, and hadles awaiting the futures itself as well.
            // trpl::join!(tx1_fut, tx_fut, rx_fut);
            // It is a major improvement compared to the previous way of doing it.
            // However, this macro requires to know the number of futures ahead of time, but in real world it could be useful to put the futures in a collection and wait for some or all of them.
            // The function `trpl::join_all` accepts a collection and iterates over and join on all of them. It accepts any type implementing the `Iterator` trait
            // let futures = vec![tx1_fut, rx_fut, tx_fut];
            // trpl::join_all(futures).await;
            // The above code doesn't compile beacuse each async block produces a `Future<Output = ()>`, so, since `Future` is a trait , the compiler creates a unique enum for each block
            // It is not possible to put two different hand written structs in `Vec`, and the same  rule applies to the enums generated by the compiler
            // To make it work the trait objects are needed, because they let treat each anonymous future produced by these types as the same type implementing the `Future` trait
            // In fact `Vec` would allow to include multiple types, but it requires an enum to represent them all, impossible since there is no way to name the different types since they are anonymous.
            // A way is to wrap each future ina a `Box::new`
            // let futures = vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
            // trpl::join_all(futures).await;
            // The above code doesn't work as well, even fixing the type error on `Box::new` by annotating the type of the `futures`:
            // let futures: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];
            // trpl::join_all(futures).await;
            // The declaration here works and it's made like this:
            // - The innermost type is the future itself, where the output is explicitly noted as `()`: `Future<Output = ()>`
            // - Then the trait is annotated as dynamic using `dyn`
            // - The entire trait is wrapped in a `Box`
            // - Finally `futures` is explicitly stated as a `Vec` containing the items
            // Now there is only the `Unpin` error to fix, which will be explained later, at the moment just follow the compiler's advice by importing `std::pin::Pin` and update the annotation for future with a `Pin` wrapping each `Box`:
            let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
                vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];
            trpl::join_all(futures).await;
            // Using `Pin<Box<T>>` from putting these futures in the heap with `Box`, and it's done just to get the types to line up: the heap allocation is not needed since the futures are all local to the function.
            // `Pin` itself is a wrapper type, so it can be used to have a single type in the `Vec`, the reason why `Box` was added, without the heap allocation
        }
        {
            // Fortunately `Pin can be used directly` using the `std::pin::pin` macro
            // Even if the type of the pinned reference must be explicit, otherwise Rust wouldn't know how to interpret the dynamic trait objects, which is what they need to be to be in the `Vec`
            // Each feature can be pinned using the macro `pin!` when it is defined, and `futures` can be defined as a `Vec` containing pinned mutable references to the dynamic futures type.

            let (tx, mut rx) = trpl::channel();
            let tx1 = tx.clone();

            let tx1_fut = pin!(async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    // Move `tx1` inside the block
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            let rx_fut = pin!(async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            });

            let tx_fut = pin!(async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    // Move the original `tx` into a new async block with different characteristics
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            });

            let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];
            trpl::join_all(futures).await;
            // In this case the `Output` type isn't considered, so, for different types, the macro `trpl::join!` can be used, but not `trpl::join_all`:
            let a = async { 1u32 };
            let b = async { "Hello!" };
            let c = async { true };

            let (a_result, b_result, c_result) = trpl::join!(a, b, c);
            println!("{a_result}, {b_result}, {c_result}");
            // Here `trpl::join_all` can't be used because it requires all of the futures to have the same type.
            // So the tradeoff is: `join_all` for a dynamic number of futures with the same type, `join!` with a set number of futures with different types, which is the same scenario as working with any other type in Rust.
        }
        {
            // When futures are joined with the `join` family of functions and macros,each of them are required to finish, but sometimes only few of them need to finish before moving on

            let slow = async {
                println!("'slow' started.");
                trpl::sleep(Duration::from_millis(100)).await;
                println!("'slow' finished.");
            };
            let fast = async {
                println!("'fast' started.");
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'fast' finished.");
            };

            trpl::race(slow, fast).await;
            // With `trpl::race` it is possible to ignore the ending of one future, changing the order of the arguments changes the print, but fast will always complete first.
            // This implementation of `trpl::race` is not fair as it always runs the futures passed as argument in the order they are passed, other implemenations are fair and choose randomly which future to run first.
            // Rust gives a runtime the chance to pause the task and switch to another one if the future awaited isn't ready.
            // The inverse is also true: Rust only pauses async blocks and hands control back to a runtime at an await point, everything between await points is synchronous.
            // This means that the work in an async block without an await point, the future will block any other futures. This is referred as starving other features, so, in a complex or long case it is useful to think about handing control back to the runtime.
            // if there is a long-running blocking operation, async can be useful for providing ways for the parts of the the program to relate each to other
        }
        {
            // How to yield control to the runtime? Let's simulate a long-runnning operation
            fn slow(name: &str, ms: u64) {
                thread::sleep(Duration::from_millis(ms));
                println!("'{name}' ran for {ms}ms");
            }
            // This code uses `thread::sleep` instead of `trpl::sleep` so slow will block the thread for some millisecs. So `slow` can be used to stand in for real-world operations, both long runnning and blocking.
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                slow("a", 10);
                slow("a", 20);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                slow("b", 10);
                slow("b", 15);
                slow("b", 350);
                trpl::sleep(Duration::from_millis(50)).await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
            // In this case each future hands control back to the runtime after carrying out a bunch of slow operations
            // `race` finishes as soon as `a` is done, without interleaving between the two futures.
            // The `a` future does all the work until `trpl::sleep` is awaited, then `b` does all its work until `trpl::sleep` call is awaited, and finally `a` completes.
            // To allow both futures to make progress between their slow tasks there is the need for await points to hand control back to the runtime, meaning there is the need of something to await
            // In the previous code, removing the `trpl::sleep` at the end of `a`, it would complete without `b` even running at all, in the following code the `sleep` is used as a starting point for letting operations switch off making progress:
            let one_ms = Duration::from_millis(1);

            let a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::sleep(one_ms).await;
                slow("a", 10);
                trpl::sleep(one_ms).await;
                slow("a", 20);
                trpl::sleep(one_ms).await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::sleep(one_ms).await;
                slow("b", 10);
                trpl::sleep(one_ms).await;
                slow("b", 15);
                trpl::sleep(one_ms).await;
                slow("b", 350);
                trpl::sleep(one_ms).await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
            // In this case `a` still runs for a bit before handing off control to `b`, because it calls  `slow` before ever calling `trpl::sleep`
            // But after the futures swap back and forth each time they hit an await point.
            // But to make progress as fast as possible the solution would be handing back control to the runtime by using `yield_now`:
            let a = async {
                println!("'a' started.");
                slow("a", 30);
                trpl::yield_now().await;
                slow("a", 10);
                trpl::yield_now().await;
                slow("a", 20);
                trpl::yield_now().await;
                println!("'a' finished.");
            };

            let b = async {
                println!("'b' started.");
                slow("b", 75);
                trpl::yield_now().await;
                slow("b", 10);
                trpl::yield_now().await;
                slow("b", 15);
                trpl::yield_now().await;
                slow("b", 350);
                trpl::yield_now().await;
                println!("'b' finished.");
            };

            trpl::race(a, b).await;
            // This code is much clearer that the one with `sleep`, and significantly faster too, because the timers such as the one used by `sleep` have limits on how granular they can be.
            // The used version of `sleep` will always sleep for at least 1ms, even if the `DUration` is 1ns, here is a benchmark
            let one_ns = Duration::from_nanos(1);
            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::sleep(one_ns).await;
                }
            }
            .await;
            let time = Instant::now() - start;
            println!(
                "'sleep' version finished after {} seconds.",
                time.as_secs_f32()
            );

            let start = Instant::now();
            async {
                for _ in 1..1000 {
                    trpl::yield_now().await;
                }
            }
            .await;
            let time = Instant::now() - start;
            println!(
                "'yield' version finished after {} seconds.",
                time.as_secs_f32()
            );
            // Here the status is not printed but the `yeald_now` is way faster compared to the `sleep` version
            // Asyncs can be useful even for compute-bound tasks, depending on what else the program is doing, because it allows to structure the relationships between different parts of the program
            // It is a form of cooperative multitasking, where each future can determine when it hands over control via await points
            // Each future has the responsibility to avoid blocking for too long and in some Rust-based embedded OSs it is the only kind of multitasking
            // In real-world code usually functions are not alternated with await calls on every single line, even because `yieald_now` is not too expansive but neither free.
            // In many cases breaking up compute bound tasks might be significantly slower compared to let an operation block intact, better measure the overall performances.
        }
        {
            // It is possible to compose multiple futures together to create new patterns, such as a `timeout` function with async blocks, the result will be another building block that can be use to create more async abstractions.

            let slow = async {
                trpl::sleep(Duration::from_millis(100)).await;
                "I finished!"
            };

            match timeout(slow, Duration::from_millis(10)).await {
                Ok(message) => println!("Succeeded with '{message}'"),
                Err(duration) => {
                    println!("Failed after {} seconds", duration.as_secs())
                }
            }
            // Because futures compose with other futures, powerful tools can be built using smaller async building blocks, e.g. timeouts with retries for network calls for example
            // The most common tools are `async`, `await` with macros such as `join`, `join_all`, and `race`.
            // Multiple futures in a sequence over time build a stream.
        }
    });

    /// Tries to run a future before the timeout elapses.
    ///
    /// # Arguments
    ///
    /// * `future_to_try: Future` - Generic future to run.
    /// * `max_time: Duration` - Maximum time to wait.
    ///
    /// # Returns
    ///
    /// * `Result<F::Output, Duration>`: If the future completes successfully it returns `Ok` with the value produced by the future,
    /// otherwise, if the timeout elapses, `Err` with the duration that the timeout waited for
    async fn timeout<F: Future>(
        future_to_try: F,
        max_time: Duration,
    ) -> Result<F::Output, Duration> {
        // Race the future passed gainst the duration, created using `thread::sleep`
        // The feature is passed first so it gets the chance to complete even if `max_time` is very short.
        // If `future_to_try` sinishes first, the `race` will return Left, otherwise `Right`
        match trpl::race(future_to_try, trpl::sleep(max_time)).await {
            Either::Left(output) => Ok(output),
            Either::Right(_) => Err(max_time),
        }
    }
}

fn streams() {
    // So far only individual futures have been considered, with the exception of async channel, where the `recv` method produces a sequence of items over time. This is an instance of a stream
    // Another sequence of items have been considered with iteretora, but the difference is that iterators are synchronous, while aync channel is asynchronous.
    // Another difference is the APIs: with iterators the synchronous method `next` is used, while with `trpl::Receiver` the asynchronous method `recv` is used.
    // These APIs are similar since a steram is basically an asynchronous form of iteration where the `trpl::Receiver` waits to receive a message and provides the next element as the `Iterator`, but asynchronously.

    use std::{pin::pin, time::Duration};
    use trpl::{ReceiverStream, Stream, StreamExt};

    trpl::run(async {
        {
            // In fact a stream can be created from any iterator, in this case an array of numbers converted to an iterator and mapped to double the value
            // Then the iterator is converted into a stream and it is looped using `while let`
            // The `next` method of the stream cannot be used if `StreamExt`, which stands for stream extension, is not in the scope. `Ext` is a common pattern in Rust for extending one trait with another
            // The `Stream` trait defines a low-level interface that combines the `Iterator` and `Future` traits.
            // `StreamExt` applies a higher-level set of APIs on top of `Stream`, including the `next` method as well as other similar to those of `Iterator`
            // `Stream` and `StreamExt` are not part of the Rust's standard library, but most of the crates use the same definition.

            let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let iter = values.iter().map(|n| n * 2);
            let mut stream = trpl::stream_from_iter(iter);

            while let Some(value) = stream.next().await {
                println!("The value was: {value}");
            }

            // With `StreamExt` in the scope all of its utility methods can be use, such as filter to only keep multiples of 3 and 5
            let values = 1..101;
            let iter = values.map(|n| n * 2);
            let stream = trpl::stream_from_iter(iter);
            let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

            while let Some(value) = filtered.next().await {
                println!("The value was: {value}");
            }
            // EVen if this could have been done with normal iterators, without async
        }
        {
            // Many concepts are naturally represented as streams: items becoming available ina queue, chunks of data pulled incfrementally froma a filesystem is too large for the memory, or data arriving from network.
            // Since streams are futures, they can be combined with any other future, for example batching events to avoid triggering too many network calls, setting timeouts on long running ooperations, or throttling UI events to avoid nedless work.
            // An example could be a stream of messages as a stand-in for a stream of data that could come from a WebSOcket, or another real-time communication protocol:
            let mut messages = get_messages();

            // Print the messages from the stream
            while let Some(message) = messages.next().await {
                println!("{message}");
            }

            // This could be used with the regular `Receiver` or `Iterator` APIs, let's add a feature such as timeout, that requires streams and applies to every element of the stream, and a delay on the items to emit.
            // Add a timeout to the stream using the `timeout` method comes from the `StreamExt` trait
            // The messages are pinned after applying the timeout because the timeout helper produces a stream that needs to be pinned
            let mut messages = pin!(get_messages().timeout(Duration::from_micros(200)));

            // the `while let` loop needs to be updated because the stream returns a `Result` now
            // `OK` indicates that the message arrived on time, `Err` that the timeout elapsed
            while let Some(result) = messages.next().await {
                match result {
                    Ok(message) => println!("{message}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                }
            }
            // Since the channel is unbounded it can hold as many messages as they can be fitted in memory, so the timeout doesn't prevent the messages to arrive
            // If the message doesn't arrive before the timeout, the stream handler will account for that, but when it polls the stream again, the messag may now have arrived
            // Different behaviours can be achieved by using other kind of channels and streams
        }
        {
            // It is possible to combine a stream of time intervals with a stream of messages, it requires a new stream of intervals called `get_intervals`
            // Let's try to merge the `messages` and `intervals`
            let messages = get_messages().timeout(Duration::from_millis(200));
            // let intervals = get_intervals();
            // let merged = messages.merge(intervals);
            // This call to `merge` which would allow to merge the two streams, doesn't work, because the two streams have different types:
            // The `messages` stream has type `Timeout<impl Stream<Item = String>>`, while `intervals` has type `impl Stream<Item = u32>`.
            // One of them needs to be transformed into the other, in our case intervals, since `messages` is already in the basic format needed
            let intervals = get_intervals()
                // Use the `map` helper to convert `intervals` into a string
                .map(|count| format!("Interval: {count}"))
                // In this case the loop never stops, so better use the `throttle` method so it doesn't overwhelm the stream of `messages`
                // Throttle limits the rate at which a function will be called, or the stream is polled in this case, in this case 100ms
                .throttle(Duration::from_millis(100))
                // Match the `Timeout` format, since it's not needed it's much longer then the others
                .timeout(Duration::from_secs(10));
            // To limit the numer of item accepted from a stream, the `take` funciton is applied on `merged`, it only pulls 20 items then stops
            let merged = messages.merge(intervals).take(20);
            let mut stream = pin!(merged);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(merged) => println!("{merged}"),
                    Err(reason) => eprintln!("Problem: {reason:?}"),
                }
            }
            // Now `throttle` produces a new stream wrapping the original, limiting the number of intervals since the orignial stream is polled at throttle rate
            // And `take` limits the numebr of messages to 20 so the program stops.
        }
    });

    /// Create an async channel over the first 10 letters of the english alphabet and send them across the channel.
    ///
    /// # Returns
    ///
    /// * `impl Stream<Item = String>`: stream of the chars generated from the channel
    fn get_messages() -> impl Stream<Item = String> {
        {
            let (tx, rx) = trpl::channel();
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
            for message in messages {
                tx.send(format!("Message: '{message}'")).unwrap();
            }
            // The type `ReceiverStream` converts the `rx`receiver from `trpl::channel` into a `Stream` with a `next` method.
            ReceiverStream::new(rx);
            // In this case, since there are no delays between messages, the timeout in the caller does not change the behaviour
        }
        {
            let (tx, rx) = trpl::channel();
            let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];

            trpl::spawn_task(async move {
                for (index, message) in messages.into_iter().enumerate() {
                    // Introduce a delay of 100ms for even indexes, and 300ms for odd indexes
                    // Since the timeout is 200ms it should affect half the messages
                    let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
                    // To sleep between messages in the `get_messages` function `async` is needed but `get_messages` can't be made async because it would chnage te return type in a `Future<Output = Stream<Item = String>>` but a `Stream` is needed
                    // In this case the caller would have to await `get_messages` to get access to the stream, but this would mean require to send all the messages, including the delay, before returning the receiver stream because inside of a future everything is linear
                    // As a result the timeout would be useless, without delays in the steram itself because they would happen before the stream was even available
                    // Instead `get_messages` returns a stream and the spawned task handles the `sleep` calls. `spawn_task` works because the runtime is already spawned, otherwise it would cause a panic.
                    trpl::sleep(Duration::from_millis(time_to_sleep)).await;
                    // tx.send(format!("Message: {message}")).unwrap();
                    // To properly send data on channel based streams errors needs to be handled, because `send` could just fail when the other channel closes,and that depends on the runtime
                    // This is handled implicitly by `unwrap` but, in a well written program, it should be managed explicitly and at minimum ending the loop
                    if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                        eprintln!("Cannot send message '{message}': {send_error}");
                        break;
                    }
                }
            });

            ReceiverStream::new(rx)
        }
    }

    /// Emit the count of intervals every millisecond using a `sleep` to send a message on a delay
    ///
    /// # Returns
    ///
    /// * `impl Stream<Item = String>`: stream of the count of intervals
    fn get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        trpl::spawn_task(async move {
            // Create the counter
            let mut count = 0;
            // Create an infinite loop
            loop {
                // Sleep for 1ms
                trpl::sleep(Duration::from_millis(1)).await;
                // Increment the count of intervals by one
                count += 1;
                // Send it over the channel
                // tx.send(count).unwrap();
                // To properly send data on channel based streams errors needs to be handled, because `send` could just fail when the other channel closes,and that depends on the runtime
                // This is handled implicitly by `unwrap` but, in a well written program, it should be managed explicitly and at minimum ending the loop
                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });
        // Since the operation is in a `trpl::spawn_task` all of it, including the loop, will be cleaned up along the runtime
        // This kind of loop is pretty common in Rust because  many programs need to run indefinitely, but with async it doesn't block anything else, as long as there is an await point in each iteration

        ReceiverStream::new(rx)
    }
}

fn traits_async() {
    // For asynchronous programming the traits used are `FUture`, `Pin`, `Unpin`, `Stream`, and `StreamExt`.
    // Here is the description in details
    {
        // `Future` trait
        use std::pin::Pin;
        use std::task::{Context, Poll};

        pub trait _Future {
            type Output;

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
        }
        // `Future`'s associated type `Output` says what the future resolves to, as `Item` for `Iterator`, so the type of value it produces once completed
        // Additionally it has the `poll` method that takes a `Pin` reference for its `self` parameter and a mutable reference to a `Context` type, and returns a `Poll<Self::Output>`
        // The `Poll` type is as follows:
        enum _Poll<T> {
            Ready(T),
            Pending,
        }
        // The `Poll` type is similar ot an `Option`: it has a variant with a value `Ready(T)`, and one without: `Pending`
        // The `Pending` variant means thatthe future still has work to do, the `Ready` means the future has finished and the `T` value is available
        // With most futures the caller shouldn't call `poll` again after the future returned `Ready` because they may panic
        // When code uses `await`, Rust compiles it to code that calls `poll`, for the `url_page_title` function, for example, it translates to something like this:
        // let mut url_page_title = page_title(url);
        // loop {
        //     match url_page_title.poll() {
        //         Ready(value) => match page_title {
        //             Some(title) => println!("The title for {url} was {title}"),
        //             None => println!("{url} had no title"),
        //         },
        //         Pending => {
        //             // continue
        //         }
        //     }
        // }
        // With the difference that `await` in this case would be blocking, so Rust makes sure that the loop can hand off control to something that can pause eork on this future to work on other futures, and check it later
        // Scheduling and coordination work is one of the main jobs of an async runtime
        // For example `rx.recv` call returns a future, and awaiting the future polls it.
        // A runtime would pause the future until it's ready with either `Some(message)` or `None` when the channel closes.
        // The runtime knows the future isn't ready when it returns `Poll::Pending`, and knows it's reaady when `poll` returns `Poll::Ready(Some(message))` or `Poll::Ready(None)`
        // The basic mechanic of futures is: a runtime polls each future it is responsible for, and put it back to sleep when it's not ready.
    }
    {
        // `Pin` and `Unpin` traits
        // The `trpl::join_all` funcion returns a struct `JoinAll` over a generic type `F` which is constrained to implement the `Future` trait
        // DIrectly awaiting a future with `await` pins the future implicitly, that's why `pin!` is not needed everywhere
        // However `trpl::join_all` constructs a new future `JoinAll` by passing a colleciton of futures to the function `join_all`
        // The signature of `join_all` requires taht the types of the items in the collection implement the `Future` trait, and `Box<T>` implements `Future` only if `T` it wraps is a future that implements `Unpin`
        // The `cx` parameter of `poll`, and its `Context` type are the key of how a runtime knows when to check any given future
        // `self` instead has a type annotation similar to any type annotations but with two differences:
        // - It tells Rust what type `self` must be for the method to be called
        // - It can't be any type, it's restricted to the type on which the method is implemented (reference or smart pointer), or a `Pin` wrapping a reference to that type
        // In order to poll a future to check whether it's `Pending` or `Ready(Output)` a `Pin`-wrapped mutable reference to the type is needed.
        // `Pin` is a wrapper for pointer-like types such as `&`, `&mut`, `Box`, and `Rc`, technically `Pin` works on types that implement `Deref` or `DerefMut`
        // `Pin` is not a pointer itself and doesn't have any behaviour by itself, it's just a tool the compiler can use to enforce constraints on pointer usage.
        // A series of await points in a future get compiled ina state machine that follow alle the normal safety rules, including borrowing and ownership
        // To make that work Rust looks at what data is needed between one await point and either the next await point or the end of the async block.
        // It then creates a corresponding variant in the compiled state machine, each variant gets the access it needs to that data used in that section of the code by taking ownership or getting a reference to it.
        // If something is wrong with ownership or reference in an async block the borrow checker will tell it.
        // Things get trickier when moving around the future that corresponds to that block, such as moving into a `Vec` to pass to `join_all`
        // When moving a future such as pushing it into a data structure or returning it from a function, means moving the state machine created by Rust
        // The future it creates for async blocks can end up with references to themselves in the field of any given variant
        // By default any object with referenceso to itself is unsafe to move because references always point to actual memory address of what they refer
        // When moving the data structure itself, the internal references will be left pointing to the old location, which is now invalid, so the value will not be updated or that memory could be used for unrelated data.
        // The Rust compiler could update every reference, but this would introduce al lot of performance overhead, especially with many references.
        // Instead making sure that data structure doesn't move in memory there would be no need to update any reference,
        // This is exactly what the borrow checker requires:in safe code it prevents from moving any item with an active reference to it.
        // `Pin` gives that exact guarantee, when a value is pinned it can no longer move, so with `Pin<Box<SomeType>>` the `SomeType` value is pinned, not the `Box`
        // The `Box` pointer can still move around becausse the important thing is making sure the data stays in place, if a pointer moves but the data it points is in the same place there is no potential problem.
        // Self-referential type cannot move because it's still pinned.
        // Most types are safe to move around, even if they are behind a `Pin` wrapper, only items with internal references require pinning.
        // For example for `Pin<Vec<String>>` everything needs to be done using `Pin` APIs even if `Vec<String>` is always safe to move if there are no references to it
        // In order to tell the compiler that an item is safe to move the trait `Unpin` is used
        // `Unpin` is  amarker trait similar to `Send` and `Sync`, so it has no functionalities by its own, they only exist to tell the compiler it's safe to use the type implementing a given trait in a particular context.
        // In particular `Unpin` informs the compiler that a give type doesn't need to uphold any guarantees about whether the value can be safely moved.
        // As `Send` and `Sync` the compiler implements `Unpin` automatically for the types it can prove it's safe, but it's possible a type doesn't implement it
        // If `SomeType` doesn't implement `Unpin` it means that it cannot safely moved in memory after being pinned, possibly because it has self references, this is expressed with this form: `impl !Unpin for SomeType`
        // There are two things to keep in mind about the relationship between `Pin` and `Unpin`:
        // - `Unpin` is the normal case, and `!Unpin` is the special case
        // - Whether a type implements one of the two only matters when using a pinned pointer to that type like `Pin<&mut SomeType>`
        // For example a `String` has a length and the chars, it can be wrapped in `Pin` and it automatically implements `Unpin`
        // On that `String` it is possible to do some operations which would be illegal if `String` implemented `!Unpin`, such as replacing a string with another
        // This doesn't violate the `Pin` contract, because `String` doesn't have internal references to make it unsafe to move around, which is the reason why it implements `Unpin` instead of `!Unpin`
        // When trying to move futures produced by an async block into `Vec<Box<dyn Future<Output = ()>>>` gives error because futures have internal references, so they don't implement `Unpin` and they need to be pinned.
        // `Pin` and `Unpin` are used for lower-level libraries, such as implementing a runtime more than day-to-day code
    }
    {
        // `Stream` trait
        // Streams are similar to asynchronous iterators but, unlike `Iterator` and `Future` they don't have definition in the standard library, even if there is a very common definition form the `futures` crate
        // Streams merge `Iterator` and `Future`: form `Iterator` there is the idea of sequence, from `Future` the idea of rediness over time
        // The definition of `Stream` could be a sequence of items that become ready over time and it's implemented like this:
        use std::pin::Pin;
        use std::task::{Context, Poll};

        trait _Stream {
            type Item;

            fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
        }

        // The `Stream` trait defines an associated type called `Item` for the type of the items produced by the stream.
        // Similarly to `Iterator` there may be 0 to many items and unlike `Future` where there is only a single `Output`
        // They also define a method to get those items called `poll_next` which polls as in `Future::poll` and produces a sequence as in `Iterator::next`
        // Its return type combines `Poll` with `Option`, the outer type is `Poll` because it has to be checked for readiness, the inner type is `Option` because it needs to signal if there are more messages.
        // In the previous example `next` and `StreamExt` were used, even if `poll_next` and `Stream` could have been used, but it's easier to use
        // `StreamExt` supplies next as follows:
        trait _StreamExt: _Stream {
            async fn next(&mut self) -> Option<Self::Item>
            where
                Self: Unpin;

            // other methods...
        }
        // In fact is slightly different  because it supports versions of Rust that did not support using async funcitons in traits: fn next(&mut self) -> Next<'_, Self> where Self: Unpin;
        // `StreamExt` is automatically implemented  for every type implementing `Stream` and has many methods to use with streams, but are defined separately to use convenience APIs without affecting the foundation trait.
        // In `trpl::StreamExt` the trait defines `next` and also supplies a default implementation of `next` that handles the datails of calling`Stream::poll_next`
        // This means that implementing a custom streaming data type requires to implement `Stream` and anyone using it can rely on `StreamExt`.
    }
}

fn futures_tasks_threads() {
    // Two approaches to concurrency have been covered: threads, and aync with futures and streams.
    // Many OSs supply threading based concurrency models, however they have their tradeoff: they use memory and come with overhead for starting up and shutting down.
    // Additionally OSs must support threads, traditional computers do, some embedded systems don't.
    // The async provides a different set of tradeoffs. In async model concurrent operations don't require threads, but they can run on tasks
    // A task is similar to a thread but it's managed by the library level code runtime instead of the OS
    // A stream can be built using an async channel and spawning an async task.
    // The same can be done using threads: instead of `trpl::spawn_task` use `thread::spawn`, and instead of `trpl::sleep` use `thread::sleep` from the std
    // For example the `get_intervals` function could become as follows:
    use std::{thread, time::Duration};
    use trpl::{ReceiverStream, Stream};

    fn _get_intervals() -> impl Stream<Item = u32> {
        let (tx, rx) = trpl::channel();

        thread::spawn(move || {
            let mut count = 0;

            loop {
                thread::sleep(Duration::from_millis(1));
                count += 1;

                if let Err(send_error) = tx.send(count) {
                    eprintln!("Could not send interval {count}: {send_error}");
                    break;
                };
            }
        });

        ReceiverStream::new(rx)
    }

    // The output of this code is the same as the already implemented `get_intervals` but is spawns OS threads instead of async tasks
    // Despite being similar they behave very differently: in modern PCs it is possible to spawn a huge number of async tasks but, with threads, they would run out of memory.
    // However the APIs are very similar for a reason:
    // Threads act as boundary for sets of synchronous operations, concurrency is possible between threads.
    // Tasks act as a boundary for sets of asynchronous operations, concurrency is possible both between ans within tasks because they can switch between futures in its body.
    // Futures are Rust's most granular unit of concurrency, and each future can represent a tree of other futures.
    // The runtime manages tasks, and tasks manage future, so tasks are similar to lightweight runtime-managed threads with the capabilities provided by the runtime instead of the OS.
    // This doesn't mean tasks are always better: threads can be simpler as they run to completion without being interrupted except by the OS
    // Threads, additionally, have no support for intratask concurrency and have no mechanism for cancellation and clean up is delegated to the OS.
    // These limitations make the threads harder to compose then futures, which are richer data structures that can be composed more naturally.
    // Additionally tasks give additional control over futures allowing to chose wehre and how to group them, additionally tasks can be moved around between threads.
    // In fact the runtime used is multithreaded by default, and use the mechanism of work stealing to move tasks around based on how the threads are used to improve the system performances.
    // The choice between them can be done following this rule:
    // - Work is very parallelizable, such as processing data where each part can be processed separately, then threads are better
    // - Work is very concurrent, such as handling messages form different sources that can come at different intervals and rates, then the better choice is async
    // if both are needed then the two can be combined as in the following example:

    // Create an async channel
    let (tx, mut rx) = trpl::channel();

    // Create a thread that takes ownership of the sender
    thread::spawn(move || {
        for i in 1..11 {
            // Send numbers from 1 to 10 then sleep for a second between each.
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Create a runtime for the future
    trpl::run(async {
        // In the future await for the messages and print them
        while let Some(message) = rx.recv().await {
            println!("{message}")
        }
    });

    // An example of scenario is runnig a set of video encoding tasks using a dedicated thread but notifying th UI that the operations are done with an async channel
}
