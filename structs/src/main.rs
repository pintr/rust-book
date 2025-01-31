/// Use of structs in Rust
/// A struct is a custom data type that lets you name and package together multiple related values that make up a meaningful group.
/// Structs are similar to tuples but in structs each piece of data has a name.
/// A struct can be  defined, instantiated, and have associated functions called methods.

struct User {
    // Example of a struct definition, having the names we don't rely only on the position like in tuples.
    active: bool, // Field, a piece of data in the struct. Each field must have a name and a type.
    username: String, // We use the String type instead of &str because we want the struct to own the data.
    email: String,
    sign_in_count: u64,
}

// A struct can store references to data owned by something else, but to do so requires the use of lifetimes.
// If we want to store references in a struct, the struct needs a lifetime annotation, otherwise the compiler will give an error.
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// Tuple structs are an alternative way to define structs when you don't need named fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct Unit; // Unit-like empty struct, useful in generics when there is need to impleent a trait but don't need any data.

fn main() {
    user_instances();
    tuple_structs();
    unit_struct();
    rectangle();
    method_syntax();
}

fn user_instances() {
    //! Function to create different instances of the User struct
    let user1 = User {
        // Basic way to create an instance of a struct, the order of the fields doesn't matter.
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };

    println!(
        "User1: {}, sign-in count {}",
        user1.username, user1.sign_in_count
    );

    let mut user2 = User {
        // We can create a mutable instance of a struct.
        // The whole instance must be mutable, not just a field.
        active: false,
        username: String::from("user2"),
        email: String::from("user2@example.com"),
        sign_in_count: 1,
    };

    user2.active = true; // We can change the value of a field of a mutable instance of a struct.

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));

    // We can create an instance of user from another instance of user.
    let _user4 = User {
        email: user2.email, // We can use the fields of another instance to create a new instance.
        ..user3 // We can use the struct update syntax to create a new instance of a struct based on another instance.
                // This must come last and only remaining fields are copied from user3.
    }; // User4 will have the same values as user3 except for the email field that will have the value of user2.email.

    // println!("User3: {}", user3.username); // user3 is not valid anymore because the field username (string) was moved to user4, so a move interaction.
    // If we only considered the active and sign_in_count fields (bool and u64), we could still use user3 because they implement the copy trait.
    println!("User3 email: {}", user3.email); // We can still use the user3 email because it was not moved out.
}

fn build_user(email: String, username: String) -> User {
    //! Function to create a User instance
    // User {
    //     // Init with the long form, having to specify twice the name of the field is a bit tedious.
    //     email: email,
    //     username: username,
    //     active: true,
    //     sign_in_count: 1,
    // };

    User {
        // Field init shorthand, if the parameter name and the field name are the same we can use the shorthand.
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_structs() {
    //! Function to show the use of tuple structs
    let black = Color(0, 0, 0); // We can create an instance of a tuple struct like a tuple.
    let origin = Point(0, 0, 0);
    // In this case balck and origin have the same values but are instances of two different types.
    // if a function expects a Color type, we can't pass an instance of Point and vice versa.

    println!("Black color: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin point: {}, {}, {}", origin.0, origin.1, origin.2);
}

fn unit_struct() {
    //! Function to show the use of unit-like structs
    let _unit = Unit; // We can create an instance of a unit-like struct.

    // We can use unit-like structs to implement traits on types that don't have any data.
}

fn rectangle() {
    //! Calculate the area of a rectangle in different ways

    {
        // Using variables
        let width = 30;
        let height = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width, height)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
    }
    {
        // Using tuples
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
    }
    {
        // Using structs
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect)
        );

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }

        // println!("rect is {rect:?}"); // The specifier :? is used to print the debug representation of a value.
        // We can't print the struct because it doesn't implement the Display trait, present in primitive types, and the trait Debug neither.
        // The Display trait is not implemented by default for structs because the way to display a struct depends on the specific use case.
        // The Debug trait can be added using hte attribute `#[derive(Debug)]` to the struct definition.
    }
    #[allow(dead_code)]
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect is {rect:?}"); // The specifier :? is used to print the debug representation of a value.
        println!("rect is {rect:#?}"); // The specifier :#? is used to print the debug representation of a value with pretty print.

        // Another way to print the Debug format is to use the `dbg!` macro, which takes ownership of an expression.
        let scale = 2;
        let rect = Rectangle {
            width: dbg!(30 * scale), // The dbg! macro will print the value of the expression and return the ownership of the value.
            height: 50,
        };

        dbg!(&rect); // Really useful for debugging, it prints the value of the expression and returns a reference to the value.
    }
}

fn method_syntax() {
    //! Methods are similar to functions, but are defined within the context of a struct (or enum or trait object).
    //! They are called on an instance of the struct and can access and modify the data of the struct.
    // Define a struct for the rectangle
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // To define a method we use the `impl` keyword followed by the name of the struct.
    // The method is defined within the context of the struct
    // It needs the first parameter to be `&self` of type Self which refers to the instance of the struct.
    // Methods can take ownership of self, borrow self immutably as we do here, or borrow self mutably, just as with any other parameter.

    // Each struct can have multiple `impl` blocks. This is useful for generic types and traits.

    impl Rectangle {
        // Methods definition

        fn area(&self) -> u32 {
            // Calculate the area of the rectangle
            self.width * self.height
        }

        // A method can have the same name of a field
        // Usually, this is done when the method is a getter of the field
        // Getters are used to access the value of a private field.
        // Unlike C and C++ where -> is used to access methods of a pointer, in Rust there is only the . operator.
        fn width(&self) -> bool {
            // Check whether the width is positive
            self.width > 0
        }

        // A method can take more than one parameter
        fn can_hold(&self, other: Rectangle) -> bool {
            // Check whether a rectangle can hold another rectangle
            self.width > other.width && self.height > other.height
        }

        // All the functions defined in the `impl` block are called associated functions.
        // Associted functions can be defined without self as a parameter, when they don't need an instance of the type.
        // E.g. String::from is an associated funciton of the String type.
        // Associated functions that aren't methods are often used for constructors that will return a new instance of the struct.
        fn square(size: u32) -> Self {
            // Create a square with sides of the `size` length
            // This method is called using the `::` syntax, like a namespace.
            Self {
                width: size,
                height: size,
            }
        }
    }

    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect.area() // Method syntax, we call the method on the instance of the struct.
        );

        if rect.width() {
            println!("The rectangle has a positive width: {}", rect.width);
        }
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(rect3));
    }
    {
        let square = Rectangle::square(10); // We call the associated function using the `::` syntax.
        println!("The area of the square is {} square pixels.", square.area());
    }
}
