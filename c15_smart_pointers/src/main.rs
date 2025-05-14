//! A pointer is a general concept for a variable that contains an address in memory.
//! The most common are references, inidcated by the `&` symbol and borrow the value they point to.
//! Smart pointers are datat structures that act as a pointer, but have additional metadata and capabilities.
//! Rust has a variety of smart pointers defined in the std,
//! For example, reference counting smart pointers that allows data to have multiple owners
//! Smart pointers own the data they point to, while references only borrow data.
//! String and Vec<T> are smart pointers as well, since they own memory and allow to manipulate it.
//! They also have addiitonal metadata, for example String stores its capacity as metadata.
//! Smart pointers are usually implemented using structs, and implement the `Deref` and `Drop` traits
//! `Deref` allows an instance of the smart pointer to behave like a reference
//! `Drop` allows to customise the code that's run when the instance of the smart pointer goes out of scope.
//! The most common in the std are:
//! - `Box<T>`: allocate values on the heap
//! - `Rc<T>`: reference counting that allow multiple ownership
//! - `Ref<T>` and `RefMut<T>`: enforces borrowing rules at runtime instead of compile time
//! Additionally interior mutability and refrence cycles are covered.

fn main() {
    box_t();
}

fn box_t() {
    // Boxes allow to store values in the heap instead of the stack.
    // On the stack only remains the pointer to the heap.
    // They are used when:
    // - A type has a size that can't be known at compile time, but a value of that type is needed in a context that requires exact size
    // - Large amount of data and there is the need to transfer ownership without copying the data
    // - Need to own a value that implments a particular trait rather than being a specific type
    {
        // Use a Box<T> to store data on the Heap
        let b = Box::new(5);
        println!("b = {b}");
        // In this case b has the value of a Box that points to the value `5` allocated on the heap.
        // Data can be accesses as it was in the stack, when the box goes out of scope it will be deallocated.
        // Since in this case it's a simple i32 value they are store by default, better using a box for complex types
    }
    {
        // Enabling recursive types with boxes
        // A value of recusrive type can have another value of the same type as part of itself
        // It's an issue because Rust needs to know how much space a type takes up, but nesting values could continue infinitely
        // Since boxes have a known size they can be used for recursive types by inserting a box in the recursive type definition
        // An example of this type is a cons list, that is made of nested pairs to form a linked list
        // For example: `(1, (2, (3, Nil)))`
    }
}
