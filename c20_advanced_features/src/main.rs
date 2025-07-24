//! The advanced features covered in this chapter are useful in very specific rare situations
//! This chapter can be seen as a references for unknowns, in particular:
//! - Unsafe Rust: how to opt out the Rus's guarantees and take responsibility for manually upholding them.
//! - Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and newtype pattern.
//! - Advanced types: newtype pattern, type aliases, never type, and dynamically sized types.
//! - Advanced functions and closures: function pointers and returning closures.
//! - Macros: ways to define code that defines more code at compile time.

fn main() {
    unsafe_rust();
    advanced_traits();
    advanced_types();
    advanced_functions_closures();
    macros();
}

fn unsafe_rust() {
    // All the code seen so far has Rust's memory safety guarantees enforced at compile time.
    // Unsafe Rust doesn't enforce them, even if it works just like regular Rust with extra features.
    // The Rust compiler is conservative as it decides whether come code upholds the guarantees or not, it rejects it.
    // The code might be okay but, if the compiler doesn't have enough information, it rejects the code.
    // The unsafe code prevents the compiler to reject it, with the risk of memory unsafety if it's incorrect.
    // Another reason Rust introduces it is that the computer hardware is unsafe inherently unsafe.
    // If Rust didn't let unsafe operations it would be impossible to do low-level programming, such as interacting directly with the OS.
    // To switch to unsafe rust is necessary to use the `unsafe` keyword and start a new block of unsafe code.
    // Unsafe blocks have five unsafe superpowers:
    // - Dereference raw pointer
    // - Call an unsafe function or method
    // - Access or modify a mutable static variable
    // - Implement an unsafe trait
    // - Access field of `union`
    // Unsafe Rust  doesn't turn off the borrow checker or other safety checks, it only gives access to the five features above.
    // Additionally, inside an unsafe block the code isn't necessarily dangerous or have memopry safety, is it just up to the programmer.
    // If there are memory safety problems it's necessarily inside an unsafe block, so better keep them small.
    // To isolate unsafe code it's best to enclose it in a safe abstraction and expose safe APIs to prevent leakages when used in safe code.
    {
        // Dereference a raw pointer
        // Usually the compiler ensures references are always valid
        // Unsafe Rust introduces two new types called raw pointers which are similar to references.
        // Raw pointers can be mutable or immutable and are written as `*const T` and `*mut T` respectively.
        // The astersik isn't the dereference operator, it's part of the type name
        // Immutable means that the pointer can't be directly assigned to after being dereferenced.
        // Differently from references and smart pointers, raw pointers:
        // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers, or multiple mutable pointers to the same location.
        // - Aren't guaranteed to point to valid memory
        // - Are allowed to be null
        // Don't implement automatic cleanup
        // This can give greater performance or the ability to interface with another language or hardware where Rust's guarantees don't apply.
        let mut num = 5;
        let r1 = &raw const num; // Immutable raw pointer
        let r2 = &raw mut num; // Mutable raw pointer
        // Raw pointers can be created in safe code, but they can't be dereferenced outside an unsafe block.
        unsafe {
            // In this unsafe block it is possible to print the values, in safe code it's not
            // In order to dereference them the dereference operator `*` is used.
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
        // If, instead of using raw pointer, created mutable and immutable references, the code wouldn't have compiled because of the ownership rules.
        // Raw pointers allow to create mutable and immutable pointers to the same location and change data, but it could end up in a data race.
        // In this case the raw pointers are created from a local variable, so they are valid, it's not always guaranteed.
        // For example, when using `as` to cast a value instead of using the raw borrow operators.
        // Here is the creation of a raw pointer to an arbitrary location in memory:
        let address = 0x012345usize;
        let _r = address as *const i32;
        // The use of arbitrary memory is undefined: there might be data in that address or not, so the compiler could optimise the code or end up in a segmentation fault.
        // Anyway, creating a pointer does no harm, its value is accessed that can end up with an invalid value.
        // The major use of raw pointers is when interfacing with C code, or building safe abstraction that the borrow checker doesn't understand.
    }
    {
        // Calling an unsafe function or method
    }
}

fn advanced_traits() {
    // There are more advanced details in traits compared to chapter 10 which are covered here.
    {
        // Associated types
        // Associated types connect a type placeholder with a trait so the trait method definitions can use the placeholders.
        // The implementor of a trait will specify the concrete type to be used instead of the placeholder.
        // in this way it is possible to define a trait that uses some types without knowing what those types are untile the trait is implemented.
        // Associated types are a commonly used advanced feature.
        // An example of a trait with an associated type is the `Iterator`:
        pub trait _Iterator {
            type Item; // Associated type, type of the values used by the type implementing the iterator

            fn next(&mut self) -> Option<Self::Item>;
        }
        // The type `Item` is a placeholder, and `next` shows it will return values of the type `Option<Self::Item>`.
        // Ihe implementors of `Iterator` will specify the concrete type for `Item`, and the `next` method will return an `Option` of that type.
        // Associated types are similar to generics with the difference that a traic can be implemented on a type a single time:
        // Generics:
        pub trait _IteratorT<T> {
            fn next(&mut self) -> Option<T>;
        }

        // In generics the types in each implementation must be annotated because it can be implemented for any type
        // For example `Iterator<String> for Counter`, so there can be multiple implementations of `Iterator` for `Counter`.
        // When a trait has a generic parameter it can be implemented for a type multiple times, changing the concrete type each time.
        // Associated types:
        struct _Counter {
            current: usize,
            max: usize,
        }

        impl Iterator for _Counter {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                return None;
            }
        }

        // Associated types, instead, don't require to annotate types because they can be implemented only once.
        // In this case the type of `Item` is selected once and it is `u32`, and the values will always be `u32`
        // Associated types become part of a trait contract: implementors of the trait must provide a type to stand in for the associated placeholder.
        // Associated types often have a name that describe how the type will be used.
    }
    {
        // Default Generic Type Parameters and Operator Overloading
        // When using generic type parameters, a default concrete type for the geneic type can be specified
        // This eliminates the need for implementors of the trait to specify a concrete type if the default one works.
        // A default type can be specified when declaring a generic type with the syntax `<PlaceholderType=ConcreteType>`.
        // An example of a situation when this is useful is with operator overloading, where hte beahviour of an operator can be customised.
        // Rust doesn't allow to create opeartors, or overload arbitrary operators, but the operations and the corresponding traits listed in `std::ops` can be overloaded.
        // This can be done by implementing the traits associated with an operator, such as overloading the `+` operator to add to `Point`.
        // This can be done by implementing the `Add` trait on a `Point` struct.

        use std::ops::Add;

        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl Add for Point {
            type Output = Point;
            // Associated type named `output` that determines the value returned by the `add` method.

            fn add(self, other: Point) -> Point {
                // Add the `x` value of the two `Point`, and the `y` as well to create a new `Point`
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 0 };
        let p2 = Point { x: 2, y: 3 };

        println!("Sum of points {:?} and {:?} equals {:?}", p1, p2, p1 + p2);

        // The generic type in the code is within the `Add` trait:
        trait _Add<Rhs = Self> {
            type Output;

            fn add(self, rhs: Rhs) -> Self::Output;
        }
        // This is a trait with one method and an assocaited type.
        // The `Rhs = Self` syntax is called default type parameter.
        // The right-hand side `Rhs` defines the type of the `rhs` parameter in the `add` method
        // If a concrete type isn't specified for `Rhs`, the default value is `Self`, which will be the type on which `Add` is implemented.
        // In the case of `Add` for `Point` there is no need to change the default `Rhs` because the behaviour was adding the two `Point` instances.
        // With two structus `Millimeters` and `Meters`, holding values in different units, the idea is to add values in millimiters to values in meters
        // The implementations of the  `Add` trait will do the conversion correctly,. It can be implementd for `millimeters` with `Meters` as the `Rhs`:
        struct Millimeters(u32);
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            // `Meters` is selected as value of the `Rhs` type instead of `Self`
            type Output = Millimeters;

            fn add(self, rhs: Meters) -> Self::Output {
                Millimeters(self.0 + (rhs.0 * 1000))
            }
        }

        // The default type parameters are used to extedn a type without breaking existing code, and to allow customisation in specific cases.
        // the standard library's `Add` trait is an example of the second purpose: usually summed data have the same type, but `Add` allows to customise that.
        // the default value for `Add` means that thre is no need to specify an extra parameter if not needed.
        // The first purpose is similar with the difference that, when assing a type paramter to an existing trait, with a default there is an extension of the functionality that doesn't break the code.
    }
    {
        // Disambiguating Between Methods with the Same Name
        // in Rust nothing prevents a trait from having a method with the same as another trait's method, nor it prevents from implementing both traits on one type.
        // It's also ossible to implement a method directly on a type, with the same name as methods form traits.
        // When this happens Rust needs to know which one is being called:
        trait Pilot {
            fn fly(&self);
        }

        trait Wizard {
            fn fly(&self);
        }

        struct Human;

        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.");
            }
        }

        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!");
            }
        }

        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*");
            }
        }
        // in this case `Human` has a `fly` method from `Pilot`, `Wizard`, and itself
        // By default the compiler calls the method riectly implemented:
        let person = Human;
        person.fly(); // This prints: *waving arms furiously*

        // In order to call the methods from the other trait, this is the syntax used:
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
        // This syntax clarifies to Rust which implementation of `fly` to use, `Human::fly(&person)` is equivalent to `person.fly()`.
        // Sinc `fly` takes a `self` parameter, with two types that implement one trait, Rust could figure out which implementation of the trait to use base on the type of `self`.
        // When types or traits define non-method funciton with the same name, Rust doesn't know which type is meant unless using a fully qualified syntax:
        trait Animal {
            // Trait with a non-method function `baby_name`
            fn baby_name() -> String;
        }

        struct Dog;

        impl Dog {
            // The `baby_name` function is directly implemented for `Dog`
            fn baby_name() -> String {
                String::from("Spot")
            }
        }

        impl Animal for Dog {
            // The `Animal`trait is implemented to `Dog` with the `baby_name` implementation
            fn baby_name() -> String {
                String::from("puppy")
            }
        }

        println!("A baby dog is called a {}", Dog::baby_name());

        // When calling `baby_name`, the function defined directly in `Dog` is called, not the one from the `Animal` trait
        // in order to print `puppy` from the trait `Animal` this is the used syntax would be:
        // println!("A baby dog called a {}", Animal::baby_name())
        // But, since `Animal::baby_name` doesn't have a `self` parameter and there could be other types that implement the `Animal` trait, Rust doesn't know which implementeation to use.
        // To disambiguate and tell Rust to use the implementation of `Animal` on `Dog`, this is the syntax:
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
        // This fully qualified syntax indicates that the `baby_name` method comes from the `Animal` trait of `Dog`.
        // In general fully qualified syntax is defined as follows: `<Type as Trait>::function(receiver_if_method, next_arg, ...);`
        // For associated functions that aren't methods, there would be no receiver, just list of arguments.
        // Fully qualified syntax can be used anywehre but, all the parts Rust can figure out, aren't needed.
    }
    {
        // Supertraits
        // Sometimes thre is the need to implement a trai that depends ona nother trait, so, to implement the first trait, the type alsorequires the other.
        // This can is done so the trait definition can make use of the associated items of the second trait.
        // The trait relied by the trait definition is called supertrait
        // For example an `OutlinePrint` trait with as `outline_print` method taht prints a value is printed in a frame of asterisks
        // Given a `Point` that implements `Display`, `outline_print` on teh value will print the frame as well.
        // To do this `Dispaly` is needed and `outlinePrint` will only work ontypes that also implement `Display`
        // This can be done in the trait definition by specifying `OutlinePrint: Display`, which is similar to adding a trait bound to a trait.
        use std::fmt;

        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string(); // To string can be used because of the required `Display` trait. Without it it would give an error.
                let len = output.len();
                println!("{}", "*".repeat(len + 4));
                println!("*{}*", " ".repeat(len + 2));
                println!("* {output} *");
                println!("*{}*", " ".repeat(len + 2));
                println!("{}", "*".repeat(len + 4));
            }
        }
        struct Point {
            x: i32,
            y: i32,
        }

        impl OutlinePrint for Point {}
        // Trying to use `OutlinePrint` on a Point gives errorbecuase it doesn't implement `Display`, to fix this `Display` must be implemented on `Point`:
        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.x, self.y)
            }
        }

        let p = Point { x: 1, y: 2 };
        p.outline_print();
        // Now it is possible to implement `OutlinePrint` and call the `outline_print` function
    }
    {
        // Using the Newtype Pattern to Implement External Traits on External Types
        // In chapter 10 was mentioned that a trait can be implemented on a type only if either the trait, the type, or both are local to the crate.
        // This restricion can be avoided using the newtype pattern, which involves creating a new type in a tuple structure.
        // The tuple struct will have one field and be a wrapper around the type for which the trait is required.
        // There is no runtime performance penality for using this pattern, and the wrapper is removed at compile time.
        // For example implementing `Display` on a `vec<T>`, which would be prevented because both are external to the crate.
        // It is possible to create a `Wrapper`  that holds an instance of `Vec<T>`, and implement `Display` on the `Wrapper`
        use std::fmt;

        struct Wrapper(Vec<String>);

        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // use `self.0` to access the `Vec<T>` because `Wrapper` is a tuple with `Vec<T>` at index 0
                write!(f, "[{}]", self.0.join(", "))
            }
        }

        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {w}");
        // The downside of using this technique is that `Wrapper` is a new type so it doesn't have methods of the value it's holding.
        // All the mothods would required to be implemented on `Wrapper` and the methods need to delegate to `self.0`
        // In this case `Wrapper` would be exaclty as `Vec<T>`.
        // If the new type requires all the methods of the inner type, implementing `Deref` on `Wrapper` to return the iiner type is a solution.
        // If it's not required to have all the methods, just some, they need to be implemented manually.
    }
}

fn advanced_types() {
    // Rust type system allow some advanced features such as newtypes, type aliases, never type, and dynamic sized types
    {
        // Using the Newtype Pattern for Type Safety and Abstraction
        // The newtype pattern is useful for tasks such as statistically enforcing that values are never confused, and indicating the units of values.
        // An example is `Millimeters` and `Meters` structs that wrapped a `u32` value in a newtype.
        // Writing a funciton with parameter `Millimeters`, the program wouldn't compile if called passing `Meters` or `u32`
        // Newtype pattern can be used to abstract some implementation details of a type, i.e. exposing a public API different from the private inner type.
        // Newtypes can also hide internal implementatoin, for example providing a `People` type to wrap an `HashMap<i32, String>` storing IDs with their names.
        // Code using `People` would only interact with the public API provided, such as adding a new element to the `People` collection.
        // The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details.
    }
    {
        // Creating Type Synonyms with Type Aliases
        // Rust also provides the ability to create a type alias to give an existing type another name.
        // To do this the `type` keyword is used:
        type Kilometers = i32;
        // In this case `Kilometers` is a synonym of `i32`, but it's not a separate, new type, they can be trated as `i32`:
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
        // Since `Kilometers` and `i32` are the same type, they can be used interchangeably, but the type-checking benefits are not available in this case.
        // in other words, mixing `Millimeters` and `i32` the compiler won't give errors.
        // They can be used for reducing repetition, for example a lenghty type such as:
        {
            let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
            // Writing this lengthy type in function signatures and as type annotations all over the code can be tiresome and error prone
            fn _takes_long_type(_f: Box<dyn Fn() + Send + 'static>) {}

            fn _returns_long_type() -> Box<dyn Fn() + Send + 'static> {
                Box::new(|| println!("hi"))
            }
        }
        // A type alias makes the code more manageable and readable:
        {
            type Thunk = Box<dyn Fn() + Send + 'static>;

            let _f: Thunk = Box::new(|| println!("hi"));

            fn _takes_long_type(_f: Thunk) {}

            fn _returns_long_type() -> Thunk {
                Box::new(|| println!("hi"))
            }
        }
        // This code is much easier to read and write, and, choosing a meaningful name, can help communicate the intent as well.
        // Type aliases are also used with the `Result<T, E>` type for reducing repetition.
        // The `std::io` module often return a `Result<T, E>` to handle situations twhen operators fail to work
        // This library has `std::io::Error` struct to represent all the I/O errors
        // many of the functions in `std::io` will be returning `Result<T, E>`, where `E` is `std::io::Error`, such as the `Write` trait:

        use std::fmt;
        use std::io::Error;

        {
            pub trait _Write {
                fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
                fn flush(&mut self) -> Result<(), Error>;

                fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
                fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
            }
        }
        {
            // The `Result<..., Error>` is repeated a lot, so `std::io` has this type alias:
            type _Result<T> = std::result::Result<T, std::io::Error>;
            // This means that `std::io::Result<T>` is a `Result<T, E>` with `E` filled in as `std::io::Error`
            // The `write` function, for example, becomes:
            pub trait _Write {
                fn write(&mut self, buf: &[u8]) -> _Result<usize>;
                fn flush(&mut self) -> _Result<()>;

                fn write_all(&mut self, buf: &[u8]) -> _Result<()>;
                fn write_fmt(&mut self, fmt: fmt::Arguments) -> _Result<()>;
            }
        }
        // the type alias helps in two ways: it makes code easier to write and gives a consistent interface across all `std::io`
        // Being an alias is just another `Result<T, E>`, meaning that any method working on `Result<T, E>` can be used, as well as the `?` operator
    }
    {
        // The Never Type That Never Returns
        // Rust has a special type named `!` that's known in type teory as the empty type because it has no values
        // It can be called the never type because it stands in the place of the return type that will never return:
        fn _bar() -> ! {
            panic!("This function never returns!")
        }
        // The function `bar` returns never, and it's called a diverging function.
        // The never type is used with `continue` too:
        // This example shows how `continue` would be used in a loop context
        let mut _guess = String::from("5");
        loop {
            let _guess: u32 = match _guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            break;
        }
        // The `match` arms must return all the same type, is `Err` would have returned a value such as a string, this wouldn't compile
        // `continue` has a `!` value: when Rust computes the type of `guess` it looks at both arms, so `u32` and `!`.
        // Beacuse `!` can never have a value, Rust declares the type of guess as `u32`.
        // Because `!` can be coerced into any type, it's allowed to end the `match` with `continue`,
        // Because `continue` doesn't return a value and it moves the control back to the top of the loop so `Err` doesn't assign a value.
        // The never type is useful with `panic!` too, since its type is `!` the result of the overall `match` expression is `T` with `Option`:
        enum _Option<T> {
            Some(T),
            None,
        }

        impl<T> _Option<T> {
            pub fn _unwrap(self) -> T {
                match self {
                    _Option::Some(val) => val,
                    _Option::None => panic!("called `Option::unwrap()` on a `None` value"),
                }
            }
        }
        // In this case Rust sees that `val` has type `T` and `panic!` has type `!`, so the result overall is `T`
        // `panic!` doesn't produce a value, it ends the program.
        // Another expression using never is `loop`:
        print!("forever ");

        loop {
            println!("and ever");
            break;
        }
        // In this case is not true because `break` terminates, without it it would never end, so the value of the expressoin is `!`.
    }
    {
        // Dynamically Sized Types and the Sized Trait
        // Rust needs to know certain details about its types, such as how much space to allocate.
        // There are dynamically sized types (DST) whose size is only known at runtime.
        // For example `str` is a DST wince the length isn't known until runtime, meaning it's not possible to create a variable of type `str` or take it as argument.
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";
        // The above code doesn't work because Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type use the same amount of memory.
        // In this case `s1` and `s2` have different lengths so it's impossible to create a variable that holds dynamically sized types
        // It's possible to fix it using a string slice `&str` instead, which just stores the starting position and the length of the slice.
        // Althought a `&T` is a single value that stores the meory address of `T`, `&str` is two values: address of `str` and its length.
        // At compile time `&str` it's twice the length of a `usize`.
        // The size of `&str` is always known, no matter how long the string is.
        // This is how synamically sized types work in Rust: they have an extra bit of metadata to store the size of the dynamic information.
        // The golden rule is that values of dynamically sized types need to be behind a pointer.
        // `str` can be combined with all kind of pointer, such as `Box<str>` or `Rc<str>`.
        // Every trait is a DST and can be referred with the name of the trait and to use them as trait objects, they need to be behind a pointer (`&dyn Trait`, `Box<dyn Trait>`, ...)
        // To work with DSTs Rust provides the `Sized` trait to determine whether or not  a type size is known at compile time.
        // This trait is implemented automatically for everything whose size is known at compile time, additionally Rust implicitly adds a bound on `Sized` to every generic function:
        {
            fn _generic<T>(_t: T) {}
        }
        // Is treated as:
        {
            fn _generic<T: Sized>(_t: T) {}
        }
        // By default generic functions will work only on types with known size at compile time, bu the following syntax allows to relax this restriction:
        {
            fn _generic<T: ?Sized>(_t: &T) {}
        }
        // A trait bound on `?Sized` means `T` may or may not be `Sized`, overriding the default that generic types must have a known size at compile time.
        // The `?Trait` syntax with this meaning is only available on `Sized`
        // The parameter `t` alsa switched from `T` to `&T` because the type might not be `Sized` so it must be put behind a pointer.
    }
}

fn advanced_functions_closures() {
    // Here are some advanced features related to functions and closures, including funciton pointers and returning closures.
    {
        // Function Pointers
        // Regular functions can be passed to functions, similarly to closures, with the difference it doesn't reuqire to redifining the closures
        // Functions coerce to the type `fn`, not to be ocnfused with `Fn` closure trait. The `fn` type is called function pointer
        // Passing functions with function pointer will allow to use functions as arguments to other functions.
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            // This function takes two parameters:
            // - A function pointer to any function that takes an `i32` and returns an `i32`
            // - A `i32` value
            // The `f` function is called twice, passing `arg` and adds up the result
            f(arg) + f(arg)
        }

        // The paramteres of `do_twice` are the function `add_one`, and `5`
        let res = do_twice(add_one, 5);
        println!("The answer is: {res}");
        // The code returns `12`
        // Unlike closures, `fn` is a type rather than a trait, so `fn` is specified as parameter directly, instead of using a generic parameter with a trait.
        // Function pointers implement all three of the closure traits: `Fn`, `FnMut`, and `FnOnce`, so a function can be passed as an argument when a closure is expected.
        // So it's best to write function using generic type and one of the closure traits, so other function can accept functions or closures.
        // This can be useful to interface with external code that doesn't have closures, e.g. C functions accept functions but C doesn't have closures.
        // Example: To use the `map` method provided by the `Iterator` trait to turn vector of numbers into a vector of strings, both closures and functions can be used:
        let list_of_numbers = vec![1, 2, 3];
        let mut list_of_strings: Vec<String> =
            list_of_numbers.iter().map(|i| i.to_string()).collect();

        println!("Result of closure:");
        for s in list_of_strings {
            println!("{s}")
        }

        list_of_strings = list_of_numbers.iter().map(ToString::to_string).collect();
        // Here the fully qualified syntax is used to specify the `to_string` function to use (from the `ToString` trait)
        // `ToString` is implemented for any type that implements `Display`.
        println!("Result of function:");
        for s in list_of_strings {
            println!("{s}")
        }
        // In enum values each variant becomes an initializer function, which can be used as funciton pointers with closure traits.
        // This means an enum variant can specify the initailizer functoins as arguments for methods that take closures:
        #[allow(dead_code)]
        {
            enum Status {
                Value(u32),
                Stop,
            }
            // `Status::Value` instances are created using each `u32` value in the range on which `map` is called
            let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        }
        // Both closures and function can be used and they compile the same code.
    }
    {
        // Returning Closures
        // Closures are represented by traits, so they can't be returned directly.
        // In most cases, when trait could be returned, instead it can be returned a concrete type that implements the trait as return value.
        // Usually it doesn't work with closures, because they don't have a returnable concrete type, so `fn` can't be used as return type for closures.
        // With the `impl Trait` syntax any function type can be returned, using `Fn`, `FnOnce`, and `FnMut`:
        fn _returns_closure() -> impl Fn(i32) -> i32 {
            |x| x + 1
        }
        // however each closure is its distintive type, so, when is needed to work with multiple functions with the same signature bu different implementation, a trait object is needed.
        fn _returns_initialized_closure(init: i32) -> impl Fn(i32) -> i32 {
            move |x| x + init
        }

        // let handlers = vec![returns_closure(), returns_initialized_closure(123)];
        // for handler in handlers {
        //     let output = handler(5);
        //     println!("{output}");
        // }
        // The code above doesn't compile because both the funciton return the same type, but the closures they return are different.
        // Whenever `impl Trait` is returned Rust creates a unique opaque type, which cannot be seen into details.
        // If both the funciotns return the same trait `Fn(i32) -> i32`, the opaque types are distinct
        // The solution is to use trait objects:
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }

        fn returns_initialized_closure(init: i32) -> Box<dyn Fn(i32) -> i32> {
            Box::new(move |x| x + init)
        }

        let handlers = vec![returns_closure(), returns_initialized_closure(123)];
        for handler in handlers {
            let output = handler(5);
            println!("{output}");
        }
    }
}

fn macros() {}
