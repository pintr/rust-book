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
        // Each item in a cons list contains two elements: the value of the current item, and the next item. The last item contains a Nil value.
        // A cons list is produced by recursively calling the cons function, the base case is Nil (not as `null` or `nil`).
        // Usually in Rust a `Vec<T>` is used instead of a cons list
        // enum List {
        //     Cons(i32, List),
        //     Nil,
        // }
        // List doesn't compile because `List` type doesn't have a known size
        // It would have been used as follows:
        // use List::Cons;
        // let list = Cons(1, Cons(2, Cons(3, Nil)));
        // The error shows that the type has inifite size saying `recursive without indirection` suggesting `insert some indirection`
        // To determine how much space to allocate for an enum, Rust goes through each variant to see which one needs more space.
        // Since only one variant will be used, the most space an enum would need is the space it would take to store the largest variant.
        // This doesn't work with a recursive type, because, in the case of List, the compiler looks at the Cons variant, which holds a i32 and a List.
        // Since it is recursive, the compiler lloks for the COns value, which holds i32 and List, and this process continues infinitely.
        // To `insert some indirection`, instead of storing a value directly, the structure should store the value indirectly by storing the pointer to the value instead.
        // Because `Box<T>` is a pointer, Rust always knows how much space a `Box<T>` uses, because a pointer size doesn't change on the amount of data it points to.
        // Modifying the Const variant using a `Box<List>` instead of List will point to the next List value on the heap, instead of another List value directly.
        // Conceptually there is still a List, butr this implementation is more like placing items next to each other instead of one insed another.
        #[derive(Debug)]
        #[allow(dead_code)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("{:?}", list)
        // The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
        // The Nil variant stores no values, so it needs less space than the Cons variant.
        // Any List will take up to the size of an i32, plus the size of a box's pointer data, breaking the inifite recursive chain.
        // Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, and they don't add performance overhead.
        // `Box<T>` type is a smart pointer because it implements the `Deref` trait, theat allows `Box<T>` values to be treated like freferences.
        // When `Box<T>` goes out of scopoe, the heap data the box is pointing at, is cleaned up as well because of the `Drop` trait.
    }
}
