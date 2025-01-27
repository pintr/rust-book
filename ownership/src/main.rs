/// Brief summary of stack and heap memory management:
/// The stack stores values in a last-in, first-out order and is faster for known, fixed-size data.
/// The heap handles dynamically sized data by allocating memory and returning pointers.
/// Stack operations are simpler and faster, but heap allocation involves more overhead due to finding and managing free space.
/// Ownership in Rust is designed to manage heap data effectively, minimizing duplicates and cleaning up unused data, ensuring efficient memory use.
/// Each value in Rust has a variable that is its owner, and there can only be one owner at a time.
/// When the owner goes out of scope, the value is dropped, freeing the memory.

fn main() {
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

    // This pattern has an impact on howRust code is written, and can result in more complicated situations.
    // In particular when multiple variables use the data allocated on the heap.
    move_interaction();
}

/// Multiple data interact with the same data in different ways
fn move_interaction() {
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
    }
}
