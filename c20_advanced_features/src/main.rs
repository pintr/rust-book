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
