/// Rust provides tools for handling duplication of concepts: generics, traits, and lifetimes.
/// Generics abstract stand-ins for concrete types and properties.
/// They allow to express the behaviour without knowing  what wil be in their place in compile time.
/// Generics are used for `Option<T>`, `Vec<T>`, `HashMap<K, V>`, Result<T,E>, but custom ones can be defined
/// There are types of functions called generic functions that allow to reduce code duplication by abstarcting the input types
/// They can be used in struct and enum too.
/// Traits are used to define behaviour in a generic way.
/// They can be used to  constrain a generic to accept only types with that behaviour.
/// Lifetimes are generics that give the compiler information about how references relate to each other.
/// This allow the compiler to ensure the borrowed values are valid

fn main() {
    generics();
    traits();
}

fn generics() {
    //! Generics create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
    {
        // Recurring operation can be extracted as a function
        // I.e. finding the largest number in a vector
        let numbers = vec![34, 50, 25, 100, 65];

        let mut largest = &numbers[0];

        for n in &numbers {
            if n > largest {
                largest = n;
            }
        }

        println!("The largest number is {largest}");

        // If the operation needs to be performed on another vector it needs to be rewritten
        let numbers2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        // So it can be extracted as a function that expects a vector of `i32` and returns a single `i32` value
        let largest2 = largest_i32(&numbers2);
        println!("The largest number is {largest2}");
    }
    {
        // It is possible that we would need a function for extracting the largest of another built-in tipe, i.e. chars
        let chars = vec!['y', 'm', 'a', 'q'];
        let largest_char = largest_char(&chars);
        println!("The largest char is {largest_char}");
        // The funciton largest char is basically the same as largest_i32, it just differs by the type
        // It is possible to parametrise a function with a generic type.
        // The type needs to be named, any name can be used, by coinvention is `T`, since it stands for `type` and it is short
        let numbers = vec![34, 50, 25, 100, 65];
        let chars = vec!['y', 'm', 'a', 'q'];

        let largest_number = largest(&numbers);
        let largest_char = largest(&chars);
        println!("The largest number is {largest_number}");
        println!("The largest char is {largest_char}");
    }

    fn largest_i32(numbers: &[i32]) -> &i32 {
        //! Function for extracting the largest number in a list
        let mut largest = &numbers[0];

        for n in numbers {
            if n > largest {
                largest = n;
            }
        }

        largest
    }

    fn largest_char(chars: &[char]) -> &char {
        //! Function for extracting the largest char in a list
        let mut largest = &chars[0];

        for item in chars {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        //! Generic function for extracting the largest element in a list
        let mut largest = &list[0];

        for item in list {
            // This control doesn't work because not all possible types allow the comparison.
            // It is possible to restrict the types valid for T to only those taht implement the trait `PartialOrd`
            // The `std::cmp::PartialOrd` trait that allows comparison
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    {
        // Generics can be used to define structs too
        struct Point<T> {
            x: T,
            y: T,
        }

        let int = Point { x: 5, y: 6 };
        let float = Point { x: 1.3, y: 4.0 };
        println!(
            "Points:\nint: {}, {}\nfloat: {}, {}",
            int.x, int.y, float.x, float.y
        );
        // Since only one generic type is specified the types can't be mixed
        // let mix = Point {x:1, y:3.5};
        // It is possible to use multiple generic type parameters
        struct PointMix<T, U> {
            x: T,
            y: U,
        }
        let mix = PointMix { x: 1, y: 3.5 };
        let same = PointMix { x: 3, y: 5 };
        println!(
            "Mixed points:\nmix: {}, {}\nsame: {}, {}",
            mix.x, mix.y, same.x, same.y
        );
        // In this case both `T` and `U` are `int32`
    }
    {
        // Generics can be used in enums as well
        // `enum Option<T>`: in Option `T` is the type of `Some`, otherwise it's `None`
        // `enum Result<T, E>`: in Result `T` is the type of `Ok`, while `E` is for `Err`
    }
    {
        // Methods can be used for methods of enums and structs
        struct Point<T> {
            x: T,
            y: T,
        }

        // This is a generic method that allows to extract the value of X for each type T
        // By declaring `impl<T>`, Rust can identify that the type in Point is a generic type
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // This method works only on Points with type f32
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let pi = Point { x: 5, y: 10 };
        println!("pi.x = {}", pi.x());
        let pf: Point<f32> = Point { x: 3.0, y: 4.0 };
        println!("Distance from origin: {}", pf.distance_from_origin());

        {
            // As seen before a struct can have multiple types, so they can have method with multiple types too.
            struct Point<X1, Y1> {
                x: X1,
                y: Y1,
            }

            // In this method a new point is returned with the x type of first, and y type of the second
            // X1 and Y1 are declared after impl because they go with the struct definition
            // X2 and Y2 are declared after mixup because they are relevant to the method
            impl<X1, Y1> Point<X1, Y1> {
                fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                    Point {
                        x: self.x,
                        y: other.y,
                    }
                }
            }

            let p1 = Point { x: 5, y: 10.4 };
            let p2 = Point { x: "Hello", y: 'c' };
            let p3 = p1.mixup(p2);

            println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
        }
    }
    {
        // Generic types don't iontroduce runtime cost because Rust performs monomorphisation of the code using generics at compile time.
        // Monomorphisation is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
        // Basically it performs the opposite of the steps used for generics.
        // Using `Option<T>` Rust in compile time reads the value used in `Option<T>` and defines specialised definitions using the specific type
        // If `Option<T>` is used for i32 and f64, the generic definition is substituted by Option_i32 and Option_f64, where some is specific, i.e. Some(i32).
    }
}

fn traits() {
    //! A trait defines the functionality a particular type has and can share with other types.
    //! Traits, similarly to interfaces, defines shared behaviour in an abstract way.
    //! A type's behaviour consists of the methods defined for that type.
    //! So different types share the same behaviour if they define the same methods
    //! Trait definitions are a way to group method signature together to define a set of behaviours necessary to accomplish some purposes.
    {
        // Considering the Summary trait and Tweet in lib
        use c10_generics_traits_lifetimes::{NewsArticle, Summary, Tweet};

        let tweet = Tweet {
            username: String::from("horse123"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet:\n{}", tweet.summarise());

        // Other crates that depend on this can bring the `Summary` trait into scope
        // The traits can only be implemented if the trait, the type, or both are local to the crate.
        // The standard library's `Display` trait could be implemented in `Tweet`
        // `Summary` could be implemented in `Vec<T>` in this crate
        // But external crate's component can't be aggregated
        // So implemented `Display` to `Vec<T>` is not allowed. This is called coherence

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        // Test the default implementation fo summarise
        println!("New article available! {}", article.summarise());
    }
    {
        // THe `impl` syntax can be used as a return value too
        // This means that the function will return a value of a type that implements the trait
        // The concrete type is not relevant
        use c10_generics_traits_lifetimes::{Summary, Tweet};

        fn returns_summarisable() -> impl Summary {
            Tweet {
                username: String::from("horse123"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            }
        }

        println!(
            "Here is the summaribable:\n{}",
            returns_summarisable().summarise()
        )
        // In this case a `Tweet` is returned, could have been any other type that implements `Summary`
        // The `impl Trait`, anyway, can be used only if a single type is return
        // fn returns_summarizable(switch: bool) -> impl Summary {
        //     if switch {
        //         NewsArticle {
        //             headline: String::from(
        //                 "Penguins win the Stanley Cup Championship!",
        //             ),
        //             location: String::from("Pittsburgh, PA, USA"),
        //             author: String::from("Iceburgh"),
        //             content: String::from(
        //                 "The Pittsburgh Penguins once again are the best \
        //                  hockey team in the NHL.",
        //             ),
        //         }
        //     } else {
        //         Tweet {
        //             username: String::from("horse_ebooks"),
        //             content: String::from(
        //                 "of course, as you probably already know, people",
        //             ),
        //             reply: false,
        //             retweet: false,
        //         }
        //     }
        // }
        // The above function doesn't work because it could return either `NewsArticle` or `Tweet`
    }
    {
        // By using a trait bound with an impl block allows to use generic parameters with specifci methods
        // For a single type it is possible to define methods available only to parameters with a specific trait
        use std::fmt::Display;

        struct _Pair<T> {
            x: T,
            y: T,
        }

        impl<T> _Pair<T> {
            fn _new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> _Pair<T> {
            // This method is available only to types that implement both DIsplay and PartialOrd
            fn _cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
        // It's even possible to implement a trait for any type that implements another trait.
        // Those are called blanket implementations
        // The standard `ToString` is defined as follows:
        // impl<T: Display> ToString for T {
        // Since this is part of the standard library, the method `to_string` defined by the trait ToString is available to anyone that implements `Display`
        let _s = 3.to_string();
        // Traits and trait bounds allow to write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
    }
}
