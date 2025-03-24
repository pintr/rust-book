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
    lifetimes();

    // All together
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }

    let string1 = "abcd";
    let string2 = "xyz";
    let ann = "Happy birthday!";

    let longest = longest_with_an_announcement(string1, string2, ann);
    println!("The longest string is: {}", longest)
}

fn generics() {
    // Generics create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
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
        // Function for extracting the largest number in a list
        let mut largest = &numbers[0];

        for n in numbers {
            if n > largest {
                largest = n;
            }
        }

        largest
    }

    fn largest_char(chars: &[char]) -> &char {
        // Function for extracting the largest char in a list
        let mut largest = &chars[0];

        for item in chars {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        // Generic function for extracting the largest element in a list
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
    // A trait defines the functionality a particular type has and can share with other types.
    // Traits, similarly to interfaces, defines shared behaviour in an abstract way.
    // A type's behaviour consists of the methods defined for that type.
    // So different types share the same behaviour if they define the same methods
    // Trait definitions are a way to group method signature together to define a set of behaviours necessary to accomplish some purposes.
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

fn lifetimes() {
    // Lifetimes ensure that the references are valid as long as needed.
    // Every reference has a lifetime, which is the scope for which the reference is valid
    // Most lifetimes are implicit, they must be annotated when the lifetimes of references could be related in a few different ways
    // The aim of lifetimes is to prevent dangling references such as:
    // {
    //     let r;                // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       | Error: `x` does not live long enough
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {r}");   //          |
    // }
    // The lifetimes are expressed an apostrophe and a letter: `'a`, `'b`
    // r is still valid, since it is declared in the scope with lifetime 'a, but its value is not ('b)
    // The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid
    // If they are not the code won't compile
    {
        // Define two string of different legth
        let string1 = String::from("abcd");
        let string2 = "xyz";

        // Get the longest one and print it
        // The function takes two string slices, wich are references, so it does
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");

        // Lifetime annotatios don't change how long any of the reference live.
        // Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
        // Functions can accept references with any lifetime by specifying a generic lifetime parameter
        // Here some examples of lifetime parameter
        // &i32        // a reference
        // &'a i32     // a reference with an explicit lifetime
        // &'a mut i32 // a mutable reference with an explicit lifetime
        // A single annotation isn't meaningful, since the lifetimes express the relation between multiple references.
        // The lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values
        // The following piece of code works because the result lifetime ends in the shortest scope, as well as string2
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }
        // The following won't, because the print command comes out of the shortest scope, when string2 is already expired
        // For this reason result is expired too
        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str()); // Error: `string2` does not live long enough
        // }
        // println!("The longest string is {result}");
        // Looking at it string1 is loger compared to string2 anyway, but the compiler can't see it
    }
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Part: {}", i.part)
        // In this case the struct has a field `part` that holds a string slice, which is a reference
        // Defining the lifetime, similarly to generics, means that the instance of teh struct can't outlive the reference it holds.
        // in this case `novel` doesn't go out of scope before `i` is used, so it is valid
    }
    {
        // There are functions where parameters and return type are references but don't need a lifetime association:
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        let novel = String::from("Call me Ishmael. Some years ago...");
        println!("First word: {}", first_word(&novel))
        // The `first_word` function would have needed a lifetime: `first_word<'a>(s: &'a str) -> &'a str`
        // But in early versions of Rust the creators found that they were entering the same lifetime annotations over and over is some cases
        // These situations were predictable and followed deterministic patterns, so they added into the compiler's code
        // More deterministc patterns are found, less lifetime annotations will be required
        // These situations are called lifetime elision rules
        // The rules aren't for the programmer to follow, but they won't require a lifetime association when encountered
        // Considering the lifetime of parameters as input lifetimes, and those on return values output lifetimes
        // The compiler uses three rules (first on input, last two on output) to figure out the lifetimes of the references, so they don't require the annotation:
        // Rule 1: the compiler assigns a lifetime parameter to each parameter that's a reference
        //         the lifetimes are different for each parameter such as:
        //         For one: `fn foo<'a>(x: &'a i32);`, for two: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);`, and so on.
        // Rule 2: if there is an explicit input lifetime parameter, it will be assigned to all the outputs: `fn foo<'a>(x: &'a i32) -> &'a i32`.
        // Rule 3: If there are multiple input parameters, but one is `&self` or `&mut self`, the lifetime of `self` is assigned to every output.
        // In the case of `first_word` teh signature is `fn first_word(s: &str) -> &str`
        // The the compiler applies the first rule: `fn first_word<'a>(s:&'a str) -> &str`
        // Then the second: fn first_word<'a>(s:&'a str) -> &'a str
        // now all references have lifetime and compiler can move on
        // Considering longest, instead, the signature is: `fn longest(x: &str, y: &str) -> &str`
        // The first ruel applies: `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str`
        // The second rule can't apply because there are more then one input lifetime parameters
    }
    {
        // The third rule only applies on method signatures
        // When implementig a method on a struct with lifetimes, declaration and use of lifetimes depend on whether they are related to struct field, or method parameters and return values
        // Lifetime names for struct fields  always need to be declared after `impl` and after the struct name, because it's part of the struct's type
        // In method signature inside impl, references might be tied to the lifetime of the fields, or may be independent
        // For the lifetime elision rules often it's not required to put  a lifetime association on method signatures, because they use `self`
        impl<'a> ImportantExcerpt<'a> {
            fn _level(&self) -> i32 {
                3
            }
            fn _announce_and_return_part(&self, announcement: &str) -> &str {
                // In this case the third rule applies, the return value has the same lifetime as `self`
                println!("Attention please: {announcement}");
                self.part
            }
        }
    }
    {
        // There is a special lifetime called `'static`, which denotes that the affected reference can live for the entire duration of the program
        // All literals have `'static` lifetime: `let s: &'static str = "I have a static lifetime.";`
        // This is because the string is stored in the binary code of the program
        // Before using `'static` it is required to think if a variable will actually live for the whole program
    }

    // fn longest(x: &str, y: &str) -> &str {
    //     // Function that returns the longer of two string slices
    //     // The function takes two string slices, wich are references, so it doesn't take ownership of the parameters
    //     // This function won't compile because it doesn't know whether `x` or `y` borrowed values are returned
    //     // In this case the concrete values passed into the function are unknown, so the case (if/else) is unknown too
    //     // For this reason the borrow checker can't determine the lifetime of the relation between `x` and `y`, and the returned value
    //     // To fix this the funciton requires a generic lifetime parameter that define the relationship between the references
    //     if x.len() > y.len() { x } else { y }
    // }

    // Structs can hold references
    // In this case they need to add a lifetime annotation on every reference in the struct's definition:
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // Function that returns the longer of two string slices using a lifetime `'a`
        // For functions, the lifetime is expressed inside angle brackets.
        // This signature expresses the contraint that the value will be valid as long as both parameters are valid
        // The generic lifetime `'a` will get the concrete lifetime that is equal to the smaller of the lifetimes of `x` and `y`
        if x.len() > y.len() { x } else { y }
    }
    // If this function only returned the first parameter, it would not be necessary to specify the lifetime on the second parameter:
    fn _longest<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }

    // When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters
    // If it doesn't the value is created in the function, generating a dangling reference:
    // fn __longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str() // Error: returns a value referencing data owned by the current function
    // }
    // Result goes out of scope at the end of the function call
    // To fix this the function should return an owned data type rather than a reference
}
