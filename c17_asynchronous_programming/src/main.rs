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
//! In Rust with `async` it's dealing cocnurrency, but it may use parallelism under the hood.

fn main() {
    futures_async();
    concurrency_with_async();
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
        // When RUst sees a function marked with `async` it compiles it into a non-async funciton whose body is an async block, the return type is the type of the anonymous data type
        {
            // The above function can be equivalently written as follows:
            // It uses the `impl Trait` syntax and the returned trait is a Future with an associated type of Output
            fn _page_title(url: &str) -> impl Future<Output = Option<String>> {
                // All the code is in a`sync move` block, which is an expression returned from the function.
                // The async block produces a value of type `Option<String>` which matches the `Output` type of the return type
                // The body is in a n `async move` block because it uses the `url` parameter
                async move {
                    let response_text = trpl::get(url).await.text().await;
                    Html::parse(&response_text)
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
    // In many cases the APIs for working with concurrency using async are very similar to those for using threads, in other cases they are similar but with a different behaviour
    // One example is spawning a new thread and make it sleep

    use std::time::Duration;

    {
        // Set runtime
        trpl::run(async {
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
        });
    }
    {
        // Async blocks compile in anonymous futures, so the runtime can run them both to completion using `trpl::join`
        // The `trpl::join` funciton is similar to `std::thread::spawn` but for futures.Giving it two futures it produces a single new future whose output is a tuple containing each future passed, and waits both to complete
        trpl::run(async {
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
        });
        // In this case the same exact order is kept every run, unlike with threads, this is because `trpl::join` is fair and checks every feature eqaully often, so it never lets one race ahead if the other is ready.
        // With threads is the OS that decides which thread to check, with async Rust is the runtime to do so.
        // Runtimes don't have to guarantee fairness for any operation, it depends on the selected runtime
    }
    {
        trpl::run(async {
            // Async allows to share messages between futures, similarly to the transfer of data between threads
            let (tx, mut rx) = trpl::channel();

            let val = String::from("hi");
            tx.send(val).unwrap();

            let received = rx.recv().await.unwrap();
            println!("Got {received}");
        });
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
        trpl::run(async {
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
        });
        // In this updated example the messages are printed at 500ms intervals rather then all togheter after 2s
        // THe program still never exits because of the way `while let` interacts with `trpl::join`:
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
        trpl::run(async {
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
            // join all three the futures
            trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        })
    };
}
