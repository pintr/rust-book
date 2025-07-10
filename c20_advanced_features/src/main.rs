//! The advanced features covered in this chapter are useful in very specific rare situations
//! This chapter can be seen as a references for unknowns, in particular:
//! - Unsafe Rust: how to opt out the Rus's guarantees and take responsibility for manually upholding them.
//! - Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and newtype pattern.
//! - Advanced types: newtype pattern, type aliases, never type, and dynamically sized types.
//! - Advanced functions and closures: function pointers and returning closures.
//! - Macros: ways to define code that defines more code at compile time.

fn main() {
    unsafe_rust();
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
