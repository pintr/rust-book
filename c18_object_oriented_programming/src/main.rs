//! Object-oriented programming (OOP) is a way of modelling programs
//! Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data.
//! The procedures are typically called methods or operations
//! Rust has some characteristics commonly considered object oriented: strcts and enums have data, and `impl` blocks provide methods.

fn main() {
    encapsulation_inheritance();
    traits_for_inheritance();
    object_oriented_design_pattern();
}

fn encapsulation_inheritance() {
    {
        // One aspect of OOP is encapsulation, where the implementation details of an object aren't accessible to code using the object
        // To interaction with the object is done using its public API.
        // Code using the object shouldn't be able to reach the object internals and change data or behaviour directly.
        // This enable the programmer to change and refactor an object's internals without having to change the code of the object
        // In Rust encapsulation is done using the `pub` keyword to decide what modules, types, functions, and methods to expose publicly, the rest is private.
        // An example is the `AveragedCollection` in lib.rs that has a list of integers and a value representing the average of that list.
        // The struct is marked `pub` so other code can use it, but the field within it remain private.
        // This is important to keep updated the average when elmeents in the list are added or removed using the `add` and `remove` functions, the `average` function gets the average.
        // The new funciton is the constructor and creates an empty `AveragedCollection`
        // The public methods `add`, `remove`, and `average` are the only ways to access or modify data in an instance of `AveragedCollection`.
        // When an item is added or removed from the list, each function calls the private `update_average` that handles the updating of the `average` as well.
        // The `list` and `average` fields are private so there is no way to update the items from the fields directly, otherwise `average` would go out of sync.
        // The `average` method return the `average` field value.
        use c18_object_oriented_programming::AveragedCollection;

        let mut collection = AveragedCollection::new();

        println!("Add 10 to the collection");
        collection.add(10);
        println!("Add 20 to the collection");
        collection.add(20);
        println!("Add 60 to the collection");
        collection.add(60);

        println!("The average is {}", collection.average());

        let value = collection.remove().unwrap();

        println!(
            "Element {value} removed, now the the average is {}",
            collection.average()
        );
        // Since the implementation details of `AveragedCollection` are encapsulated, aspects of it can be changed in the future.
        // For example using an `HashSet<i32>` instead of a `Vec<i32>` for the `list` field.
        // As long as the signature of the public methods remains the same, code using it doesn't need to change.
        // If `list` was public and the type changed, since `HasSet<i32>` and `Vec<i32>` have different methods for adding and removing items, the external code would need to change
    }
    {
        // Inheritance is a mechanism where an object can inherit elements from another object's definition
        // The object would gain parent object's data and behaviour without having to define it again
        // Rust can do it by using a macro
        // The main reasons to use inheritance are:
        // - Reuse of code: the bahaviour is implemented for one type, inheritance enables to reuse that iomplementation for another type
        // - Polymorphism: enable a child type to be used the same places as the parent type, which means that multiple objects can be substituted by others at runtime if they share certain characteristics.
        // inheritance may share more code than necessary: subclasses don't need to share all the characteristics of the parent, but they do.
        // This leads to make a program design less flexible, and introduces the possibility of calling method on subclasses that don't make sense, or produce an error.
        // Rust takes the different approache of using trait objects instead of inheritance
    }
}

fn traits_for_inheritance() {
    // In chapter 8 an enum was defined to store different value types in a vector.
    // However it may be useful to let user extend the set of types that are valid
    // An example is a GUI tool, called `gui`, that lets the user draw GUI tools, some tools may be `Button` or `TextField` but other can be added too, such as `Image` or `SelectBox`
    // The `gui` library needs to keep track of the values of different types it needs to call the `draw` method on each of these typed values.
    // It's not important what the `draw` function does, just that the value has the method available.
    // In a language with inheritance this could be done by defining a class `Component` that has a method `draw` on it.
    // The other classes, such as `Button`, would inherit from `Component` and thus inherit the `draw` method too, that could be overriden to define a custom behaviour.
    // Since Rust doesn't have inheritance traits are used to allow users to extend the `gui` library.
    // To implement this behaviour a trait called `Draw` with a method `draw` is needed, then it is possible to define a vactor that takes a trait object.
    // A trait object points to both an instance of a type implementing a trait, and a table to llok up trait methods on that type at runtime.
    // A trait object can be created by specifying some sort of pointer, such as `&` or `Box<T>`, then the `dyn` keyword, and specifying the trait.
    // A trait object can be used in place of a generic or concrete type and Rust will ensure at compile time that any value used in the context implements that trait, without knowing all the possible types.
    // Trait objects, differently from structs or enum that require `impl to specify a behaviour, combine data and behaviour.
    // The purpose of a trait object is to allow abstraction across common behaviour
    // So, the `gui` module needs a trait `Draw` with a method `draw`, and a struct `Screen` that holds a vector called components of type `Box<dyn Draw>`
    // `Box<dyn Draw>` is a trait object and allows to insert any element that implement the `Draw` trait
    // On the `Screen` struct it's defined a method `run` that calls the `draw` method on each of the `components`
    // This is different from defining a struct that uses a generic type parameter with trait bounds, because it can only be substituted with one concrete type at a time, not multiple

    {
        // An example of this implementation would be the following:
        use c18_object_oriented_programming::gui::Draw;
        pub struct _Screen<T: Draw> {
            pub components: Vec<T>,
        }

        impl<T> _Screen<T>
        where
            T: Draw,
        {
            pub fn _run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
        // This restricts the `Screen` instance to use a list of components all of the same type (such as `Button`, or `TextField`)
        // Anyway, with an homogeneous collection, using generics and trait bounds is preferable because the definitions are monomorphised at compile time.
        // On the other hand with the method using trait objects can hold a `Vec<T>` that contains `Box<Button>` as well as `Box<TextField>`
    }
    // In the `gui` library a button has been created implementing the `Draw`trait
    // The fields on Button (`width`, `height`, and `label`) may differ from other components, such as `TextField` which would add a `placeholder`
    // Each of the types will implement the `Draw` trait, but the method `draw` is different for each of them, they could even have additional `impl` blocks containing methods realted to other events (e.g. click of button)
    // Here is the implementation of a `SelectBox` using `Draw`:

    use c18_object_oriented_programming::gui::{Button, Draw, Screen};

    {
        #[allow(dead_code)]
        struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }

        impl Draw for SelectBox {
            fn draw(&self) {
                // Draw the select box
            }
        }
        // Here is the `Screen` instance used for adding the components and draw the using the `run` function, which will call the `draw` method of each component:
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };

        screen.run();
    }
    // When the `gui` library was written the added components aren't known, such as `SelectBox`, but the `Screen` implementation allows it since it works with the `Draw` trait.
    // Similarly, when `screen.run()` is called it doesn't need to know what the concrete type of each component is, it just calls the `draw` method,which is present as specified by the `Box<dyn Draw>` type.
    // For example adding another element that doesn't implement it, results in an error:
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };
    // screen.run();
    // The error is the following: the trait `Draw` is not implemented for `String`
    // The compiler generates nongeneric implementations of functions and methods for each concrete type used in place of a generic type parameter.
    // The code that results from monomorphisation does static dispatch: the compiler knows the method called at compile time
    // In this case, instead, dynamic dispatch is used: the compiler doesn't know at compile time which concrete method is called, the compiler emits code that it is figured out at runtime.
    // At compile time the compiler doesn't know all the types that might be used, so the method to call, instead at runtime Rust uses the pointer inside the trait object to know which method to call.
    // Dynamic dispatch prevents prvents the compiler from choosing to inline a method's code, which prevents some optimisation, and Rust has rules called syn compatibility, about where dynamic dispatch can be used.
    // However this introduces flexibility, but the trade-off is to consider.
}

fn object_oriented_design_pattern() {}
