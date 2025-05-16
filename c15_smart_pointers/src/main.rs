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
    defer_trait();
    drop_trait();
    reference_counting();
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

fn defer_trait() {
    // Implementing the Defer trait allows to customise the behaviour of the dereference operator *
    // Using the Defer trait permits to treat smart pointers like regular references, allowing to write code  that works on references and use that code with smart pointers too.
    // A regular reference is a type of pointer,a nd a pointer is an arrow to a value stored somewhere else, the dereference operator follows teh arrow to get the value
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // x holds value 5, y is a reference to x, sot to get the value it needs to be dereferenced
    // This code can be rewritten using a `Box<T>`
    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // The difference is that y is an instance of `Box<T>` pointing to a copy of the value of 5, rather than a reference to the value of x
    // it is possible to define a smart pointer similar to `Box`
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    // MyBox is a struct with a generic parameter `T`, while the MyBox type is a tuple struct with one element of type T.

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
    // MyBox<T> can't be dereference because that ability is not implemented, the `Deref` trait is needed.

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T; // Associated type for the `Deref` trait to use

        // Associated  types are a slightly different  way of declaring a generic parameter

        /// Return a reference to the value to be accessed with the * operator
        fn deref(&self) -> &Self::Target {
            &self.0 // Access the first value in a tuple struct
        }
    }

    assert_eq!(5, *y); // Now it works

    // Without the `Deref` trait the compiler can only deference & references.
    // `*y` actually runs the following code: *(y.deref())
    // Rust substitutes the * operator with a call to the deref `method`
    // `deref` returns a reference to a value because, if it returned the value directly, the value would move out of `self`, moving the ownership
    // The * operator is replaced witha  call to deref and then a call tot he * operator just once, each time * is used. The substitution of * does not recurse infinitely.

    // Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type
    // For example it can convert `&String` to `&str` because `String` implements the `Deref` trait
    // Deref coercion was added so programmers don't need  to add many explicit references, it also lets writing more code that works for either references or smart pointers.
    // Here an example
    fn hello(name: &str) {
        println!("Hello {name}");
    }
    // `hello` can be called with a string slice: `hello("Rust");` or, thanks to deref coercion, with a reference to a value of type `MyBox<String>`
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // `hello` can be called with the argument `&m`, which is a reference to a `MyBox<String>`.
    // Because of the `Deref` trait, Rust can turn `&MyBox<String>` into `&String` by calling deref. Std provides an implementation of `Deref` on `String` that returns a slice.
    // Rust calls `deref` to turn `&String` into `&str`, if it wasn't for deref coercion the code would have been as follows:
    hello(&(*m)[..]);
    // (*m) dereferences the `MyBox<String>` into a `String`, then `&` and `[..]` take a string slice of the `String`.
    // When the `Deref` trait is defined for the types involved, Rust abnalyses the types and use `Defer::defer` as many time as needed to get a reference to match the parameter's type.
    // The number of times is resolved at compile time, so there is no runtime penality.

    // `DerefMut` is used to override the * operator on mutable references
    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // - From `&T` to `&U` when `T: Deref<Target=U>`
    // - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
    // - From `&mut T` to `&U` when `T: Deref<Target=U>`
    // The first two cases are the same except for the mutability. If there is a `&T` and `T` implements `Deref` to some type `U`, &U can be obtained transparently.
    // The third case shows that a muitable reference can be coerced to an immutable one, but not vice versa.
    // There can be only a single reference to some data, because of the borrowing rules. Converting an immutable to a mutable reference breaks the borroing rule.
    // Converting an immutable reference to a mutable one would require that the immutable reference is the only to that data, but Rust can't guarantee it.
}

fn drop_trait() {
    // The second trait useful for smart pointers is `Drop`, that allows to customise what happens when a value is about to go out of scope.
    // The functionality of the `Drop` trait is almost always used when implementing smart pointers, for example when `Box<T>` is dropped, it will deallocate the space on the heap.
    // In many languages freeing operations is done manually every time, in Rust the behaviour can be specified once using the `Drop` trait, and the compiler will add it automatically.
    // The `Drop` trait requires to implement the method `drop` that takes a mutable reference to self:
    struct CustomSmartPointer {
        data: String,
    }

    // The `Drop` trait is included in the prelude, so there is no need to bring it into scope.
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            // Print the following when the `CustomSmartPointer` is dropped.
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("Created data c: {} and d: {}", c.data, d.data);

    // As soon as `c` and `d` go out of scope, they print the content of the `drop` method
    // It is possible that is needed to drop a value early, for example when smart pointers manage locks.
    // Rust doesn't let to call the `drop` method manually, instead the functione needed is `std::mem::drop`:
    // c.drop(); // Doesn't work
    drop(c); // Use `std::mem::drop`
    println!("Early drop of c!");

    // The code specified in the `Drop` trait can be used to make cleanup convinient and safe
    // Additionally the ownership system makes sure references are always valid, and the `drop` function is called only once when the value is no longer being used.
}

fn reference_counting() {
    // There are cases when a single value might have multiple owners.
    // For example in a graph multiple edged might point to the same node, which is conceptually owned by all of them, so it should be clean only when it doesn't have any edges.
    // Multiple ownership must be explicitly enabled using the type `Rc<T>`, which stands for reference counting.
    // `Rc<T>` keeps track of the numebr of references to a value to determine if it is still in use, if there are zero references it can be cleaned.
    // `Rc<T>` is used when some data allocated on the heap is used by multiple parts of the program, but can't be determined at compile time which part finishes last, otherwise the normal ownership rules would apply.
    // `Rc<T>` is only for use in single threaded scenarios and can be used to share data between two structure, for example the cons list:
    {
        #[derive(Debug)]
        #[allow(dead_code)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let a = Cons(5, Box::new(Cons(10, Box::new(Nil)))); // List shared between `b`, and `c`
        let b = Cons(3, Box::new(a)); // Valid because `a` is moved into `b`, and becomes its owner.
        println!("b: {:?}", b)
        // let c = Cons(4, Box::new(a)); // Not valid because `a` has been moved into `b`
        // One way to fix this would be changing the definition of `Cons` to hold references, but then the lifetime would be required.
    }
    {
        // Another way of managing is changing the definition of `List` using `Rc<T>` instead of `Box<T>`.
        // In this case each `Cons` variant will hold a n`Rc<T>` value pointing to a `List` and, when `b` is created, instead of taking ownership of `a`, the `Rc<List>` hold by `a` is cloned.
        // In this case the number of references increases, letting `a` and `b` to share ownership of the `Rc<List>`, allowing to create and share ownership with `c` too.
        // The reference count increases each time `Rc::clone` is called, and the data won't be cleaned up until the data within `Rc<List>` has no references to it

        #[derive(Debug)]
        #[allow(dead_code)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use std::rc::Rc;
        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // List shared between `b`, and `c`
        let b = Cons(3, Rc::clone(&a)); // The `Rc<List>` in `a` is cloned, allowing multiple ownership
        let c = Cons(4, Rc::clone(&a));
        println!("a: {:?}, b: {:?}, c: {:?}", a, b, c);
        // `Rc<T>` is not in the prelude, so it needs to bi brought into scope.
        // We could have used `a.clone()` instead of `Rc::clone` but the convention is to use the latter, because most implementations of `clone` make a deep copy of all the data, while `Rc::clone` only increases the reference count.
        // To check the reference counting `Rc` provides the method `Rc::strong_count`:
        {
            let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
            println!("Count after creating a = {}", Rc::strong_count(&a));
            let _b = Cons(3, Rc::clone(&a));
            println!("Count after creating b = {}", Rc::strong_count(&a));
            {
                let _c = Cons(4, Rc::clone(&a));
                println!("Count after creating c = {}", Rc::strong_count(&a));
            }
            println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
        }
        // In this example the reference count can be seen as it increases and decreases base on the `clone` and `drop` operations
        // `Rc<T>` allows to share data between multiple parts of the program reading only
        // If `Rc<T>` allowed modifying it would violate the borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies.
    }
}
