//! Object-oriented programming (OOP) is a way of modelling programs
//! Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data.
//! The procedures are typically called methods or operations
//! Rust has some characteristics commonly considered object oriented: strcts and enums have data, and `impl` blocks provide methods.

use c18_object_oriented_programming::AveragedCollection;

fn main() {
    encapsulation_inheritance();
    traits_for_inheritance();
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

fn traits_for_inheritance() {}
