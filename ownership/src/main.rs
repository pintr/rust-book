/// Brief summary of stack and heap memory management:
/// The stack stores values in a last-in, first-out order and is faster for known, fixed-size data.
/// The heap handles dynamically sized data by allocating memory and returning pointers.
/// Stack operations are simpler and faster, but heap allocation involves more overhead due to finding and managing free space.
/// Ownership in Rust is designed to manage heap data effectively, minimizing duplicates and cleaning up unused data, ensuring efficient memory use.
/// Each value in Rust has a variable that is its owner, and there can only be one owner at a time.
/// When the owner goes out of scope, the value is dropped, freeing the memory.

fn main() {
    ownership();
    move_interaction();
    assign_interaction();
    clone_interaction();
    ownership_and_functions();
    return_scope();
    references_borrowing();
    dangling_references();
    slice_problem();
    string_slices();
}

fn ownership() {
    //! Ownership basics in Rust
    {
        // s not valid here, not yet declared
        let _s = "hello"; // s is a string literal, which is immutable and fixed in size.

        // do stuff with s
    } // scope over, s no longer valid

    // See the rules of ownership with the String data type, which is more complex than the already seen types that can be stored on the stack.
    // String literals are immutable and hardcoded into the program executable.
    // The String type is allocated on the heap, which allows for a mutable, growable piece of text.
    // With the String type the amount of memory is unknown at compile type
    // In this case the memory must be at runtime from the memory allocator, which returns a pointer to the heap. Universal in programming languages.
    // After the String is no longer needed, the memory must be returned to the allocator.
    // In most languages there is the garbage collector that cleans up memory not being used anymore.
    // In Rust the memory is automatically returned once the variable that owns it goes out of scope.
    // This is done by Rust using the drop fuction, which is called automatically at the closing curly brace of the variable's scope.
    {
        let mut s = String::from("hello"); // namespace the from function under the String type
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{s}");
    } // s goes out of scope here, and the memory is returned to the allocator

    // This pattern has an impact on how Rust code is written, and can result in more complicated situations.
    // In particular when multiple variables use the data allocated on the heap.
}

fn move_interaction() {
    //! The ownership of a value is transferred to a new variable.
    {
        // In this example there is a copy of the value x to y, both are 5
        let x = 5;
        let _y = x;
    }
    {
        // This example is similar to the previous but, using a String type
        // A String is made of three parts:
        // - Pointer to memory: pointer to where the content is stored on the heap.
        // - Length: how much memory in bytes the contents of String are currently using.
        // - Capacity: amount of memory in bytes that the String has received from the allocator.
        // The group of data is stored on the stack, while the content is stored on the heap
        let s1 = String::from("hello");
        let _s2 = s1;

        // In this case s1 creates a String structure with pointer, length, and capacity.
        // When s2 is assigned to s1, the pointer, length, and capacity are copied, but not the data on the heap.
        // Both s1 and s2 point to the same memory location.
        // This means that, when s1 and s2 go out of scope, the drop function will free the same memory location twice.
        // This error, called double free, is a memory safet bug, that can lead to security vulnerabilities.
        // Rust prevents this error by invalidating the first variable, s1, when the second variable, s2, is assigned.

        // println!("{s1}, world!"); // This doesn't work because s1 is invalidated

        // This is called a move in Rust, most of the other languages, instead, make shallow copies of the data.
    }
}

fn assign_interaction() {
    //! When a new value is assigned is assigned to an existing variable, Rust drops the original value's memory immediately.
    let mut s = String::from("hello");
    println!("{s}, world!");
    s = String::from("aohy");

    println!("{s}, world!");
}

fn clone_interaction() {
    //! To make a deep copy of the heap data, the clone method can be used.
    //! This method is used to copy the heap data, and not just the stack data.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Basic data types which are entirely stored in the stack since they have a known size at compile time, are always deep copied.
    // Rust has a special annotation for types stored on stack, called Copy trait.
    // if a type implements the Copy trait, variables that use it by default are copied.
    // If a type, or part of it, implements the Drop trait, Rust won't let annotate a type with Copy.
}

fn ownership_and_functions() {
    //! Passing a variable to a function will move or copy, depending on the type.
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and so is no longer valid here

    // println!("{s}"); // This doesn't work because s is no longer valid

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
    println!("{x}"); // This works because x is still valid

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
    } // some_string goes out of scope and `drop` is called. The backing memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{some_integer}");
    } // some_integer goes out of scope. Nothing special happens.
}

fn return_scope() {
    //! Returning values from functions can also transfer ownership.
    let _s1 = gives_ownership(); // gives_ownership moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope
    let _s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        let some_string = String::from("hello"); // some_string comes into scope
        some_string // some_string is returned and moves out to the calling function
    }

    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }

    // Since assigning a value to another variable moves the ownership, this works but, taking ownership and returning it for every function, is very tedious.
    // Rust has a feature called references, which allow to refer to a value without taking ownership of it.
}

fn references_borrowing() {
    //! References allow to refer to a value without taking ownership of it.
    // A reference behaves like a pointer, so it's an address that can be followed to access data stored at that address.
    // A reference is guaranteed to point to a valid value of a particular type for the life of the reference.
    // When a reference is used there is no need to return the value because the function never had ownership.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    // By default references are immutable
    // change(&s1);
    // In order to modify the reference, it must be mutable, as follows:
    let mut s = String::from("hello");
    change_mut(&mut s);

    // There can be only one mutable reference to a value.
    let r1 = &mut s;
    let r2 = "";
    // let r2 = &mut s; // This doesn't work
    println!("{r1}, {r2}");

    // This is done because Rust prevents data races at compile time.
    // A data race happens when the following behavior occurs:
    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There's no mechanism being used to synchronize access to the data.
    // Data races are unpredictable, and difficult to diagnose and fix, for this reason Rust prevents them by refusing to compile code.
    // There can be multiple mutable references but not simultaneously: instead in different scopes.
    {
        let mut s = String::from("hello");

        {
            let _r1 = &mut s;
        } // r1 goes out of scope here, so we can make a new reference with no problems.

        let _r2 = &mut s;
    }

    // A similar rule is carried out when combining mutable and immutable references.
    // There can be multiple immutable  references, because they just read data, without affecting anyone else.
    // Just one mutable reference would mean that the data gets modified, changing the value for immutable references too.
    #[allow(unused_mut)]
    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = "";
        // let r3 = &mut s; // Error: cannot reference mutable and immutable references at the same time

        println!("{}, {}, and {}", r1, r2, r3);
    }

    // A reference scope starts from where it is introduced and continues through the last time that reference is used.
    // This means that, after the last usage of immutable references, a mutable one can be declared
    #[allow(unused_mut)]
    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{r1} and {r2}");

        let mut r3 = &mut s;
        println!("{r3}");
    }
    // In this case the immutable references r1 and r2 end after the println!, allowing the creation of r3.

    fn calculate_length(s: &String) -> usize {
        // The signature uses & as a reference of a string
        s.len()
    }

    #[allow(dead_code)]
    fn change(_s: &String) {
        // Like variables references are immutable, so it is not possible to modify something we have a reference to.
        // _s.push_str(", world!"); // This doesn't work
    }

    fn change_mut(s: &mut String) {
        //! To allow modifying the reference, the reference must be mutable
        s.push_str(", world!");
    }
}

#[allow(unused_variables)]
fn dangling_references() {
    //! Languages with pointers allow to create dangling pointers, which are pointers that reference a location in memory that may have been given to someone else.
    //! Rust prevents this by enforcing the rules of ownership.
    // The compiler ensures that data won't go out of scope before the reference to the data does.
    // let reference = dangle();
    let reference = no_dangle();

    // fn dangle() -> &String { // Returns a reference to a String
    //     let s = String::from("hello"); // The new string
    //     &s // Reference to s is returned
    // } // s goes out of scope, and is dropped. The reference to s is now invalid.

    fn no_dangle() -> String {
        let s = String::from("hello"); // String is created

        s // Ownership is moved out, so s is valid
    }
}

fn slice_problem() {
    //! Slices are references to a contiguous sequence of elements in a collection.
    //! They don't have ownership, and are used to reference a portion of a collection.

    // For example return the first word of a string with multiple words separated by spaces.
    {
        let mut s = String::from("hello world");
        let w = first_word(&s);
        s.clear(); // This empties the String, making it equal to ""

        // w is still valid, but the value is no longer valid, so it is out of sync with the data in s
        println!("{w}");
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes(); // Convert the string to an array of bytes to check element by element

            for (i, &item) in bytes.iter().enumerate() {
                // Create an iterator over the array of bytes, and enumerate it to get the index and the value
                if item == b' ' {
                    // If the value is a space return the index
                    return i;
                }
            }
            // Otherwise return the length of the string
            s.len()
        }
    }
}

fn string_slices() {}
