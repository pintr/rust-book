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
        // Dereferencing a raw pointer
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
        // The second type of operation that can be performed in an unsafe block is calling unsafe functions.
        // Unsafe functions and methods look like regular functions and methods but with an extra `unsafe` in the definition
        // The `unsafe` keyword indicates the funciton has requirements that need to be upholded when the function is called, because Rust can't guarantee it.
        // Calling an unsafe function in an `unsafe` block means the user take responsibility for upholding the funciotn contracts:
        unsafe fn dangerous() {}
        // In order to run the unsafe `dangerous` function, it's necessary to call it in an unsafe block, otherwise it gives error.
        unsafe {
            dangerous();
        }
        // With the `unsafe` block it s being asserted that the docs of the function have been read, and it is known how to use it properly to fulfill the contract.
        {
            // Creating a Safe Abstraction over Unsafe Code
            // Just because a function contains unsafe code, it doesn't mean the entire function needs to be unsafe.
            // It is possible to wrap unsafe code in a safe function, which is a pretty common abstraction.
            // An example is the `split_at_mut` function of the standard library which requires unsafe code.
            // The safe method is defined on mutable slices, and it takes one slice and makes it two by splitting the slice at the index given:
            use std::slice;

            let mut v = vec![1, 2, 3, 4, 5, 6];
            println!("Original vec: {:?}", v);

            let r = &mut v[..];

            let (a, b) = r.split_at_mut(3);

            println!("a: {:?}", a);
            println!("b: {:?}", b);
            // `split_at_mut` can't be implemented only using safe Rust as follows (thinking of it as a function for `i32` values instead of a method)
            // fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            // Get length of `values`
            // let len = values.len();
            // This asssertion panics if `mid` isn't less or equal to `len`
            // assert!(mid <= len);
            // the returned value is two mutable slices ina tuple: one from the start to `mid`, the other from `mid` to the end
            // (&mut values[..mid], &mut values[mid..])
            // }
            // Rust's borrow checker can't nuderstand that different parts of the slices have been borrowed, it only knows that the same slice has been borrowed twice
            // It would be okay to borrow different parts of a slice since they don't overlap, but Rust doesn't know it.
            // For this reason the implementation is made using unsafe code:

            fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
                // Slices are a pointer to some data and the length of the slice.
                // The `len` method is used to get the length of a slice and the `as_mut_ptr` method to access the raw pointer of a slice.
                let len = values.len();
                // In this case, since there is a mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with type `*mut i32`, stored in the variable `ptr`
                let ptr = values.as_mut_ptr();

                // The assertion that the `mid` index is within the slice is kept.
                assert!(mid <= len);

                unsafe {
                    (
                        // The unsafe code, `slice::from_raw_parts_mut` funciton, takes a raw pointer and a length, and creates a slice.
                        // It is used to reate a slice that goes from `ptr` and is `mid` items long.
                        slice::from_raw_parts_mut(ptr, mid),
                        // The `add` method on raw pointers is also unsafe because it must trust that the offset locatoin is also a valid pointer.
                        // Therefor the calls to `slice::from_raw_parts_mut` and `add` must be in a `unsafe` block to call them.
                        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                    )
                }
                // If the assertion `mid <= len` is true, all the raw pointers within the `unsafe` block will be valid pointers to data within the slice.
                // this is an appropriate use of `unsafe`
            }
            // It is not required to mark the resultant `split_at_mut` as unsafe, and can be called from safe Rust
            // This is the creation of a safe abstraction to unsafe code with the implementation of the function that uses `unsafe` code in a safe way
            let (a, b) = split_at_mut(r, 3);

            println!("a: {:?}", a);
            println!("b: {:?}", b);

            // Instead, the following use of `slice::from_raw_parts_mut` would likely crash because it takes an arbitrary memory location and creates a slice with 10000 items.

            // let r = 0x01234usize as *mut i32;
            // let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
            // The memory at this arbitrary location isn't own by the program, so there is no guarantees that the slice contains valid `i32` values, attempting to use it fails
            // println!("{:?}", _values);
        }
        {
            // Using `extern` Functions to Call External Code
            // Sometimes Rust needs to interact with code written in another language
            // For this reason Rust introduces the `extern` keyword to facilitate the creation and use of a Foreign Function Interface (FFI).
            // An FFI is a way for programming languages to define funcitons and enable a different programming language to call the functions.
            // Functions within `extern` are generally unsafe to call from Rust code, so `extern` blocks must be marked as`unsafe`
            // This is required because other programming languages don't enforce Rust's rules and guarantees, and Rust can't check them.
            // Here is an example of the use of `abs` from the C standard library:
            {
                unsafe extern "C" {
                    // Here are listed the names and signatures of external functions from another language.
                    // The `"C"` part defines which applation binary interface (ABI) the external function uses
                    // The ABI defines how to call the function at assembly level
                    // The `"c"` ABI is the most common and follows the C programming language's ABI, others are available.
                    fn abs(input: i32) -> i32;
                }

                unsafe {
                    println!("Absolute value of -3 according to C: {}", abs(-3));
                }
            }
            // Every item declared within `unsafe extern` is implicitly `unsafe`, even if some FFI are safe to call.
            // For example `abs` from C doesn't have any memory safety considerations, and can be called with any `i32`
            // In cases like this the `safe` keyword can be used to say taht a specific funciton is safe to call, even if it is in an `unsafe extern` block.
            // Whan made safe, it is not necessary to call it from an `unsafe` block anymore:
            {
                #[allow(clashing_extern_declarations)]
                unsafe extern "C" {
                    // MArking a function as `safe` doesn't make it safe inherently, it's like a promise to Rust that it is.
                    safe fn abs(input: i32) -> i32;
                }

                println!("Absolute value of -3 according to C: {}", abs(-3));
            }
            // `extern` can be used to create an interface that allows other languages to call Rust functions
            // instead of creating a whole `external` block, `extern` can be added while specifying the ABI to use before `fn`
            // It also require the annotation `[unsafe(no_mangle)]` to tell Rust not to mangle the name of the function.
            // Mangling is when a compiler changes the name given to a function to adifferent name with more information for other parts of the compilation process to consume.
            // Mangled functions are less human readable, and every programming languagemangle names slightly differently.
            // in order to use a Rust funciton in another language the mangling of the Rust compiler must be disabled.
            // This is unsafe because it could collie across libraries, so it is a user responsibility to make sure the chosen anme is safe to export without mangling.
            // Here is an example of function accessible from C code after it is compiled:
            #[unsafe(no_mangle)]
            pub extern "C" fn call_from_c() {
                println!("Just called a Rust function from C!");
            }
            // This usage of `extern` requires unsafe only in the attribute, not on the `extern` block
        }
    }
    {
        // Accessing or Modifying a Mutable Static Variable
        // Rust allows to use global variables, which are supported but may be problematic with Rust's ownerhip rules
        // If two threads are accessing the same mutable global variable, it can cause a data race.
        // in Rust global variables are called static varaibels as shown here:
        static HELLO_WORLD: &str = "Hello, world!";

        println!("Name is: {HELLO_WORLD}");
        // Static variables are similar to constants, and are annotated using the `SCREAMIN_SNAKE_CASE` by convention.
        // Static variables only store references with the `'static` lifetime, so Rust can figure out the lifetime and it's not required to explicitly annotate it.
        // A difference between constants and immutable static variables is tahat values in a static variable have a fixed address in memory
        // Using the value will always access the same data.
        // Constants, instead, are allowed to dubplicate their data whenever they're used
        // Static varaibles can be mutable, but accessying and modifying them is unsafe, here an example:
        // As regular varaibles they must have the `mut` keyword
        static mut COUNTER: u32 = 0;

        // Every code that reads or writes from `COUNTER` must be within an `unsafe` block.
        /// SAFETY: Calling this from more than a single thread at a time is undefined
        /// behavior, so you *must* guarantee you only call it from a single thread at
        /// a time.
        unsafe fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        unsafe {
            // SAFETY: This is only called from a single thread in `main`.
            // This prints at first `COUNTER: 0`, then edits the value, finally it prints `COUNTER: 3` as expected because it's single threaded
            // With multiple accesses it would likely end up in data races and unefined behaviour.
            // Therefor the entire function must be marked as `unsafe`, and document the safety limitation writing a comment starting with `SAFETY`
            // it's not possible to create references to a utable static variable, it can only beaccessed via a raw pointer, created with one of the raw borrow operators.
            // Even if the reference is created indisibly, such as in `println!`
            // The need of raw pointers help make the requirements for using them more obvious.
            println!("COUNTER: {}", *(&raw const COUNTER));
            println!("Add 3");
            add_to_count(3);

            println!("COUNTER: {}", *(&raw const COUNTER));
        }
        // With mutable data globally accessbile, it's difficult to ensure there are no data races, which is why Rust consider mutable static variables unsafe.
        // Where possible is better to use the concurrency techniques and thread-safe pointers, so the compiler checks that data access is doen safely by different threads.
    }
    {
        // Implement an unsafe trait
        // An trait is unsafe when at least one of its methods has some invariant the compiler can't verify
        // The trait is declared as `unsafe` by adding the `unsafe` keyword before `trait`, and marking the implementationa s `unsafe` too:
        unsafe trait _Foo {
            // methods go here
        }

        // By using `unsafe impl`, the invariants thatthe compoiler can't verify are upholded
        // For example `Sync` and `Send` marker traits: the compiler implements these traits automatically if the types are composed entirely of other types that implement `Send` and `Sync`.
        // Implementing a type that contains a type without `Send` or `Sync`, such as raw pointers, and they need to be marked as `Send` or `Sync`, `unsafe` must be used.
        // Rust can't verify that the type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads, so the check must be done manually.
        unsafe impl _Foo for i32 {
            // method implementations go here
        }
    }
    {
        // Accessing fields of a Union
        // The final action that only works with `unsafe` is accessing fields of a union.
        // A `union` is similar to a `struct`, but only one declared field is used in a particular instance at one time.
        // Unions are primarily used to interface with unions in C code.
        // Accessing union field is unsafe because Rust can't guarantee the type of the data currently being stored in the union instance.
    }
    // When writing unsafe code, it may be useful to check whether the code is correct and safe.
    // Rust offers an official tool called Miri to detect undefined behaviours.
    // Miri is a dynamic tool that works at runtime, it checks the code byrunning the program, and detects the violation of the rules it understands.
    // For example `cargo +nightly miri run`
    // Miri issues warnings whenundefined behaviour is found, without telling how to fix it.
    // miri can also detect outright errors, so patterns that are wrong for sure, and make reccomendations on how to fix them.
    // Miri doesn't catch everything that might be wrong, since it is a dynamic analysis tool, it catches the problems with code that actually runs.
    // it is used in conjunction with good testing to increase confidence about the unsafe code.
    // So, if Miri catches a problem, there's a bug, but if it doesn't catch it it doesn't mean there isn't a problem.
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

fn macros() {
    // macros have been used in the other chapters, but were never fully explored.
    // The term macro refers to a family of features in Rust: declarative macros with `macro_rules!`, and procedural macros:
    // - Custom `#[derive]` macros that specify code added with the `derive` attribute on structs and enums
    // - Attribute-like macros that define custom attributes usable on items
    // - Function-like macros that look like funciton calls but operate on token specified as their argument
    {
        // The Difference Between Macros and Functions
        // Macros are a way of writing code that writes other code (metaprogramming)
        // The `derive` attribute generates an implementation of various traits
        // The `println!` and `vec!` macros alsa have been used. All of these macros axpand to produce more code than written
        // Metaprogramming is useful for reducing the amount of code to write and maintain, similarly to funcitons, but with addiitonal powers.
        // A function signature must declare the number and type of paramters, macros, instead, can take a variable number of paramters (e.g. `println!`)
        // Macros, also, are expanded before the compiler interprets the meaning of code, so a macro can, for example, implment a trait on a type.
        // A function can't do that because it gets called at runtime, and a trait needs to be implemented at compile time.
        // the downside of macros is that macro definitions are more complex than function definitions, because it's Rust code that writes Rust code.
        // Due to this indirection, macro efinitions are more difficult to read, understand, and maintain compared to functions.
        // Another important difference is that macros must be defined or brought into scope before they are called, functions, instead, can be defined anywhere and call anywhere.
    }
    {
        // Declarative Macros with macro_rules! for General Metaprogramming
        // The most widely used macros in Rust are the declarative macros, referred as "macros by example", "`macro_rules!` macros", or plain " macros".
        // Declarative macros allow to write something similar to `match` expressoins, which are structure taht take an expression, compare the resultant value to patterns, and run the code associated to the pattern.
        // Macros also compare a value to patterns that are associated with particular code, in this case:
        // - The value is the Rust source code passed to the macro;
        // - The patterns are compared with the structure o the source code;
        // - The code associated with each pattern, when matched, replaces the code passed to the macro.
        // This all happens during compilation
        // To define a macro the construct `macro_rules!`, here explained how it works using `vec!` as a reference:
        let _v: Vec<u32> = vec![1, 2, 3];
        // The `vec!` macro is used to make a vector, in this case of three integers, but could be anything else.
        // it wouldn't be possible using a function because the number or type of values are not known.
        // here is a semplified version, without pre-allocatoin for optimisation, of the definition of `vec!`:

        // The `#[macro_export]` annotation, uncommented, inidcates that the macro should be made available whenever the crate in which is defined is brought into scope. Otherwise it can't be brought into scope.
        // #[macro_export]
        // here starts the macro definition using `macro_rules!` and the name of the macro `_vec` (should have been `vec`), and curly brackets denoting the body.
        macro_rules! vec {
            // the structure in the `_vec!` body is similar to the structure of a `match` expression.
            // There is one arm with pattern `( $( $x:expr ),* )` followed by `=>` and the code associated with the pattern.
            // if the pattern matches, the associated code will be emitted. In this case there is only one arm.
            // Valid pattern syntax in the macro definitions is different from the pattern syntax because macros are matched against Rust code structure rather than values.
            // First a set of parentheses is used to encompass the whole pattern.
            // A dollar sign `$` is used to declare a variable in the macro system taht will contain the Rust code matching the pattern.
            // The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable.
            // Next the set of parentheses capture the values that match the pattern within them for use in the replacement code.
            // Within `$()` is `$x:expre`, which matches any Rust expression and gives the name `$x`.
            // The comma following `$()` indicates that a literal comma separator character must appear between each instance of the code that matches the code within `$()`.
            // The `*` specifies taht the pattern matches zero or more of whatever precedes the `*`.
            // When the macro is called with `vec![1, 2, 3]`, the `$x` pattern matches three times with the three expressions `1`, `2`, and `3`
            ( $( $x:expr ),* ) => {
                {
                    // The mutable temp_vec is defined and will be returned
                    let mut temp_vec = Vec::new();
                    // `temp_vec.push()` within `$()*` is generated for each part that matches `$()` in the pattern zero or more times, depending on how many times the pattern matches.
                    $(
                        // The `$x` is replaced with each expression matched.
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }

        let _v = vec![1, 2, 3];
        // With `vec![1, 2, 3]` the code generated is the following:
        // {
        //     let mut temp_vec = Vec::new();
        //     temp_vec.push(1);
        //     temp_vec.push(2);
        //     temp_vec.push(3);
        //     temp_vec
        // }
        // So it has been generated a macro that can take any number of arguments of any type, and can generate code to create a vector containing the specified elements.
    }
    {
        // Procedural Macros for Generating Code from Attributes
        // the second form of macros is the procedural one, which acts like a function, and is a type of procedure.
        // procedural macros accept some code as input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code.
        // the three kinds of procedural macros are: custom `derive`, attribute-like, and function-like, and all work in a similar way.
        // When creating procedural macros, the definitions must reside in their own crate with a special crate type.
        // Here is an example where `some_attribute` is a placeholder for using a special macro variety.
        // use proc_macro;

        // #[some_attribute]
        // pub fn some_name(input: TokenStream) -> TokenStream {
        //     input
        // }
        // The funciton that defines a procedural macro takes a `TokenStream` as an input, and produces a `TokenStream` as an output.
        // the `TokenStream` type is defined by the `proc_macro` crate, included with Rust, and represents a sequence of tokens.
        // This is the core of the macro: the source code that the macro is operating on makes the input `TokenStream`, and the code the macro produces is the output `TokeStream`
        // The function also has an attribute attached to it that specifies which kind of procedural macro is being created. There can be multiple procdural macros in a crate.
    }
    {
        // How to Write a Custom derive Macro
        // Here is an example of a custom `derive` macro.
        // A trait named `HelloMacro` is being created with one associated function named `hello_macro`.
        // Rather than implementing the `HelloMacro` trait, a procedural macro is provided, so user can annotate their type with `#[derive(HelloMacro)]` to get an implementation of the `hello_macro` function.
        // The default implementation will print `Hello, Macro! My name is TypeName!` where `TypeName` is the name of the type on which this trait has been defined.
        // In other words it will write a crate that enables another programmer to write code as follows:
        // use hello_macro::HelloMacro;
        // use hello_macro_derive::HelloMacro;

        // #[derive(HelloMacro)]
        // struct Pancakes;

        // fn main() {
        //     Pancakes::hello_macro();
        // }
        // The code will print `Hello, Macro! My name is Pancakes!`
        // The code should be in lib.rs, instead is in main for compatibility with the macro.
        // Now there is a trait and is function, so it can be implemented to achieve the functionality:
        // Define the `HelloMacro` trait and its associated function `hello_macro`
        pub trait HelloMacro {
            fn hello_macro();
        }

        {
            struct Pancakes;

            impl HelloMacro for Pancakes {
                fn hello_macro() {
                    println!("Hello, Macro! My name is Pancakes!");
                }
            }

            Pancakes::hello_macro();
        }
        // A user should write the implementation block for each type to be used, this needs to be spared.
        // Currently the function `hello_macro` with default implementation that will print the name of the type can't be provided yet.
        // Rust doesn't have reflection capabilities so it can't look up the type's name at runtime, a macro is required to generate code at compile time.
        // The next step is to define the procedural macro in lib.rs
        // The convention for structuring crates and maco crates is as follows: for a crate named `foo`, a custom `derive` procedural macro crate is called `foo_derive`
        // In this case the lib in the `c20_advanced_features` crate will be used.
        // If the trait definition changes, the implementation of the procedural macro needs to be changed as well.
        // The two crates will need to be publiched separately, and programmers using them will need to add both as dependencies and bring them both in the scope.
        // It would be possible to add the `derive` as a dependency to the crate, and re-export the procedural macro code,
        // Publishing them separately allows to use just `hello_macro` without `derive`.
        // the `derive` crate (in this case lib.rs) is declared as a procedural macro crate.
        // Additionally the funcitonality from `syn` and `quote` crates need to be added as dependencies in `Cargo.toml`
        // the procedural macro is impelemnted in the `lib.rs` file, which stands as the `hello_macro_derive` crate.
        // The code is split into the `hello_macro_derive` function, responsible for parsing the `TokenStream`, and the `impl_hello_macro` function, responsible for transforming the context tree.
        // Transforming the syntax tree makes writing a procedural macro more convenient.
        // The code in the outer function `hello_macro_derive` will be the same fo almost every procedural macro crate.
        // The code in the inner function `impl_hello_crate` will be different depending on the procedural macro purpose.
        // Three new crates have been introduced:
        // - `proc_macro`: come with Rust, so it's not a dependency, and it is the compiler's API that allows to read and manupulate Rust code from the code.
        // - `syn`: parses Rust code from a string into a data structure taht can be operated on.
        // - `quote`: turns `syn` data structures back to Rust code, making it simpler to parse Rust code, which is a very difficult task.
        // The `hello_macro_derive` function will be called when a user of the library specifies `#[derive(HelloMacro)]` on a type.
        // This is possible because the `hello_macro_erive` has been annotated with `proce_macro_derive`, and specifies the name `HelloMacro` which mathces the trait name.
        // The `hello_macro_derive` funciton converts the `input` form `TokenStream` toa  data structure taht can be interpreted and performed operations on.
        // `syn` comes into play by using the `parse` function to take a `TokenStream` and return a `DeriveInput` struct representing the parsed Rust code.
        // here an example of the `DeriveInput` struct from parsing the `struct Pancakes;` string:
        // DeriveInput {
        // // --snip--

        // ident: Ident {
        //     ident: "Pancakes",
        //     span: #0 bytes(95..103)
        // },
        // data: Struct(
        //     DataStruct {
        //         struct_token: Struct,
        //         fields: Unit,
        //         semi_token: Some(
        //             Semi
        //         )
        //     }
        // )
        // }
        // The fields of this struct shows that the parsed Rust code  is a unit struct with the `ident` (identifier) of `Pancakes`, but there are more fields.
        // in the `impl_hello_macro` function there is the Rust code that needs to be included, but before the output of `derive` is also a `TokenStream`
        // The returned `TokenStream` is added to the code written by the user so, when the crate is compiled, it will contain the extra funcitonality provided in the modified `TokenStream`.
        // The `unwrap` function is used to cause the `hello_macro_derive` funciton to panic if the call `syn::parse` fails.
        // It's necessary for the procedural macro to panic on errors because `proc_macro_derive` functions must return `TokenStream` rather than `Result` to conform to the procedural macro API.
        // For this reason it has been simplified by using `unwrap;`, in produciton code a more specific error message should be provided by using `panic!` or `expect`.
        // At this point `cargo build` should complete successfully in both `hello_macro`, and `hello_macro_derive`
        // Runnoing the following code, it successfully prints: `Hello, Macro! My name is Pancakes!` by using the implementeation of the `HelloMacro` trait from the procedural macro.
        use c20_advanced_features::HelloMacro;

        #[derive(HelloMacro)]
        struct Pancakes;

        Pancakes::hello_macro();
        // The `#[derive(HelloMacro)]`  added to the trait implementation allows to use the macro on `Pancakes`
    }
    {
        // Attribute-Like macros
        // Attribute-like macros are similar to custom `derive` macros, but instead of generating code for the `derive` attribute, they allow you to create new attributes.
        // They are also more flexible: `derive` only works for structs and enums, while attributes can be applied to other items as well, such as functions.
        // Here is an example of an attribute-like macro: an attribute named `route` taht annotates functions when using a web app framework:
        // #[route(GET, "/")]
        // fn index {}
        // The `#[route]` attribute would be defined by the framework as a procedural macro, the signature of the macro definition funciotn would be like:
        // #[proc_macro_attribute]
        // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
        // Here there are two paramters of the type `TokenStream`:
        // - The contest of attribute: the `GET, "/"` part
        // - The body of the attribute is attached too: in this case `fn index {}`, and the resto of the function's body.
        // Other than that, attribute-like macros work the same way as custom `derive` macros: a crate `proc-macro`crate type can be created aimplementing a function that generates the wanted code.
    }
    {
        // Function-Like macros
        // Function-like macros define macros that look like function calls.
        // Similarly to `macro_rules!` macros, they are more flexible than functions, as they can take an unknown number of arguments, for example.
        // However, `macro_rules!` macros can only be defined using the match-like syntax.
        // Function-like macros take a `TokenStream` parameter, and their definition manipulates that `TokenStream` using Rust code ad the other two types of procedural macros do.
        // An example of a funciton-like macro is `sql!`:
        // let sql = sql!(SELECT * FROM posts WHERE id=1);
        // This macro would parse the SQL statement inside it, and chack that's syntattically correct, which is more complex porcessing than a `macro_rules!`.
        // The `sql!` macro would be defined as follows:
        // #[proc_macro]
        // pub fn sql(input: TokenStream) -> TokenStream {}
        // The deifnition is similar to the custom `derive` macro's signature: the tokens inside the parentheses are received, and the generated code is returned.
    }
}
