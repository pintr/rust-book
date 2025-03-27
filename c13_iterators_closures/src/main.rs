//! Rust implements many features of functional programming
//! Functional programming includes using functions as arguments, return values, or assigning to variables for later execution
//! In particular Rust provides:
//! - Closures: function-like construct that can be stored in a variable
//! - Iterators: a way of processing a series of elements
//! These features are added to c12_minigrep as improvement

use std::{thread, time::Duration};

fn main() {
    closures();
    iterators();
}

fn closures() {
    // Rust closures are anonymous functions that can be saved in a variable or pass as argument
    // Closures can be created in a place and called elswhere in a different context
    // Unlike functions, closures can capture values from the scope they are defined
    /// Variant for the color of a shirt
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Blue,
        Red,
    }
    /// Inventory of shirts of different colours
    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            // Get the colour of the shirt based of the preference, or the most stocked one
            // It uses the `unwrap_or_else` method on `Option<T>` defined by the standard library
            // This method takes one argument: a closure without arguments that returns a value of type T
            // If the `Option<T>` is in the `Some` variant, it returns that value, otherwise the closure is called
            // In this case `most_stocked` is called, that returns the correct type `ShirtColor`
            // The standard library doesn't know anything about `Inventory` or `ShirtColor`
            // The closure just captures the `self Inventory` instance
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            // Get the most available colour in the inventory
            let mut nb = 0; // Number of blue
            let mut nr = 0; // Number of red

            for color in &self.shirts {
                match color {
                    ShirtColor::Blue => nb += 1,
                    ShirtColor::Red => nr += 1,
                }
            }

            if nb > nr {
                ShirtColor::Blue
            } else {
                ShirtColor::Red
            }
        }
    }
    {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }
    {
        // There are many differences between functions and closures
        // Closures don't require to annotate types of parameters and return value
        // In function type annotations are required  because they are explicitly exposed
        // Closures aren't exposed, tey are stored in variables without naming
        // CLosures are typically short and relevant within a context, so the compiler can infer the types
        // Anyway closures allow the type annotation, as follows:
        let _expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };
        // Here are different syntaxes compared to a function
        // fn add_one_v1(x: u32) -> u32 { x + 1 }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x| { x + 1 };
        // let add_one_v4 = |x| x + 1;
        // v3 and v4 require the closures to be evaluated to be able to compile
        // because the types will be inferred from their usage
        let example_closure = |x| x;
        // In this case any type would be good, but only one can be selected
        let _s = example_closure(String::from("hello"));
        // let _n = example_closure(5); // if `_s` commented ok, if `_s` uncommented error
    }
    {
        // Closures can capture values from their environment in three ways, like a function:
        // Borrowing immutably: the closure only captures an immutable reference
        // Borrowing mutably: the closure captures a mutable reference (and updates it)
        // Taking ownership: the closure takes total ownership of the element
        {
            // Borrow immutably: no need to update value for printing
            let list = vec![1, 2, 3];
            println!("Before defining closure: {list:?}");

            let only_borrows = || println!("From closure: {list:?}");

            println!("Before calling closure: {list:?}");
            only_borrows();
            println!("After calling closure: {list:?}");
        }
        {
            // Borrow mutably: the closure adds a value to a list
            let mut list = vec![1, 2, 3];
            println!("Before defining closure: {list:?}");

            let mut borrows_mutably = || list.push(7);

            borrows_mutably();
            println!("After calling closure: {list:?}");
        }
        {
            // Taking ownership: list is taken by another thread and printed
            let list = vec![1, 2, 3];
            println!("Before defining closure: {list:?}");
            // `move` converts a variable captured by reference to a variable captured by value
            // This is necessary because it is not known which thread will finish first
            thread::spawn(move || println!("From thread: {list:?}"))
                .join()
                .unwrap();
        }
        {
            // Once a closure has captured  a reference or ownership of a value from the environment where it is defined
            // The body of the closure defines what happens to references or values when it's evaluated
            // A closure can move a value out of the closure, mutate the value, or neither
            // The way a closure handles values affect which trait it implements, multiple can be implemented:
            // 1. `FnOnce`: applies to closure that can be called one, all closures implement this.
            //              If the closure moves the value out, it only implement this trait
            // 2. `FnMut`: applies to closures that don't move the value out, but mutate the captured value.
            //             They can be called more than once.
            // 3. `Fn`: applies to closures that don't move captured values out and don't mutate the captured values.
            //          They can be called more than once and don't mutate the environment (i.e. for concurrency)
            // The fokllowingg is the definition of `unwrap_or_else`:
            // impl<T> Option<T> {
            //     pub fn unwrap_or_else<F>(self, f: F) -> T
            //     where
            //         F: FnOnce() -> T,
            //     {
            //         match self {
            //             Some(x) => x,
            //             None => f(),
            //         }
            //     }
            // }
            // `T` is the geenric type of the value of the `Some` variant, and the return value of the function
            // There is the generic type `F`, which is the closure provided by the user
            // it has the trait `FnOnce`, so it must be able to be called once, takes no arguments, and return `T`
            // This means that `f` will be called at most one time. Since has only `FnOnce` it accepts all three kinds of closures
            #[derive(Debug)]
            struct Rectangle {
                width: u32,
                height: u32,
            }

            let mut list = [
                Rectangle {
                    width: 10,
                    height: 1,
                },
                Rectangle {
                    width: 3,
                    height: 5,
                },
                Rectangle {
                    width: 7,
                    height: 12,
                },
            ];
            // Here is the `sort_by_key` closure
            // it uses the `FnMut` trait: it takes an argument in form of a reference to the current item, and returns value `K`
            // list.sort_by_key(|r| r.height + r.width);
            // println!("{list:#?}");
            // `sort_by_key` takes a `FnMut` closure because it calls the closure multiple times, and `|r| r.height + r.width` doesn't capture, mutate, or move anything
            // To check how many times here is another closure with a counter:
            let mut num_sort_operations = 0;
            // let mut sort_operations = vec![];
            // let value = String::from("closure called");
            list.sort_by_key(|r| {
                // sort_operations.push(value);
                num_sort_operations += 1;
                r.height + r.width
            });
            println!("{list:#?}, sorted in {num_sort_operations} operations");
            // In the closure it is not possible to push a value into a mutable vector defined outside, because the closure would take owenrship, meaning it would implement `FnOnce`
            // Since the closure needs to be called multiple times it can't implement `FnOnce`, additionally, after one iteration, the value pushed in teh vector wouldn't be valid
        }
    }
}

fn iterators() {
    // The iterator pattern allows to perform some task on a sequence of items in turn.
    // It is responsible for the logic of iterating ove each item, and terminate when finished
    // In Rust iterators are lazy, so they have no effect until they are consumed
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // In this case the iterator is created and stored in v1_iter, it can be used in many ways.
    // For exampèle through a `for` loop:
    for item in v1_iter {
        println!("Got {item}");
    }
    // In the case of a `for` the iterator would be implicitly created:
    for item in v1 {
        println!("Got {item}");
    }
    // All iterators implement a trait called `Iterator` which has the folowing definition
    pub trait _Iterator {
        // Defining an associated type with this trait (Chapter 20)
        // Item type is used in the returned type of the `next` method, so of the Iterator
        type Item;
        // The `next` method returns one item of the iterator at a time, wrapped in `Some`, when it ends it returns `None`
        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
    let v1 = vec![1, 2, 3];
    // `v1_iter` must be mutable because calling the `next` method changes internal state used by the iterator to keep track where it is
    // Each `next` call eats up an item from the iterator, in the `for` loop is not required becaus it takes ownership and does it behind the scenes
    // The values returned by `next` is immutable
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // The `Iterator` trait has a number of methods with default implementations provided by the standard library
    // Some call the `next` method in their definitio, these methods are called consuming adapters
    // An example is the `sum()` method, which takes ownership of the iterator and iterates by calling the `next` funciotn repeatedly
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    // Iterators adapters, instead, are methods defined on the iterator, but they don't consume it, instead they produce a different iterator
    // For example the `map` method takes a closure to call on each item, and returns the updated iterator:
    let v1: Vec<i32> = vec![1, 2, 3];
    // The `collect` method consumes the iterator and collects the resulting values in a collection data type
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    // Since `map` takes a closure , any operation can be performed, even a chain of calls to iterator adapters
    // Since many iterator adapters take closures as arguments, these closures can capture the environment of the iterator
    // For example the `filter` method takes a closure and returns a bool for each item in the iterator.
    // if it's `true` the value will be included in the resulting iterator, otherwise not
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // This function takes ownership of a vector of shoes and a shoe size as parameters.
        // It returns a vector containing only shoes of the specified size.
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
    // The decision to use loops or iterators depends on which implementation is faster
    // The two implementations have similar performance because, although a high-level abstraction, compile to roughly the same code as loops
    // They are called zero-cost abstractions, as they don't impose additional runtime overhead
}
