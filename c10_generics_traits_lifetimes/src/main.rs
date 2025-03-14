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
}

fn generics() {
    {
        // Before diving into generics recurring operation can be extracted as a function
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
