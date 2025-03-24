/// Rust standard library provides a number of data structures called collections.
/// These collections point to data stored in the heap, which means the data can grow and shrink at runtime.
/// The most common collections are: vectors, strings, and hash maps.

fn main() {
    vectors();
    strings();
    hash_maps();
    excercises();
}

fn vectors() {
    // Vectors are similar to arrays, but they can grow or shrink in size.
    // Vectos can only store elements of the same type.
    // Create empty vector, since it's empty the type annotation is required.
    let _v: Vec<i32> = Vec::new();
    // When a vector is created with initial values, the type annotation is not required, since it is inferred.
    // Additionally, Rust provides the `vec!` macro to create a vector with initial values.
    let mut v = vec![1, 2, 3];
    // To add elements to a vector, the `push` method is used, the vector must be mutable.
    v.push(4);
    v.push(5);
    // There are two ways to reference a value stored in a vector: via indexing or by using the `get` method.
    // Use of indexing
    let third: &i32 = &v[2];
    // In this case using & and [] gives a reference to the element at that index.
    println!("The third element is {third}");
    // Use of the `get` method
    let third: Option<&i32> = v.get(2);
    // In this case using `get` returns an Option<&T> which is a reference to the element at that index.
    // The match statement is used to handle the Option returned by `get`.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }
    // When trying to access an element that is out of bounds, the program panics with indexing, but with `get` it returns `None`.
    // let _does_not_exist = &v[100]; // This will panic
    let _does_not_exist = v.get(100); // This will return None
                                      // This is usefult, for example, when the index is provided by the user.

    // When a reference to an element in a vector is created, the reference is dropped when the vector is modified.
    // This happens because the borrow checker ensures that all references are valid.
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // Mutable borrow occurs here
    println!("The first element is: {first}");
    // This error is due to the way vectors work: adding a new element at the end of the vector requires allocating new memory and copying the old elements to the new space.
    // If there isn't enough memory to put all elmeents next to each other the refence to the first element would be pointing to deallocated memory.

    // It is possible to iterate over the elements of a vector using a for loop.
    for i in &v {
        println!("{i}");
    }
    // Additionally, there is the possibility to modify the elements iterating over mutable references.
    for i in &mut v {
        *i += 50; // To change the value it's necessary to dereference the value, to get the value in the vector, instead of a reference.
    }

    for i in &v {
        println!("{i}");
    }
    // it is not possible to inser or remove elements from a vector while iterating over it. because of the borrow checker.

    // Vectors can only store elements of the same type, but it is possible to store different types using an enum.
    #[derive(Debug)]
    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }
    // Rust needs to know what types are possible in the vector at compile time, so it can allocate enough space.
    // It's needed because performing operations to any data without knowing the type will bring an error.
    // Using an enum and a match witll ensure that the types are known at compile time.

    // Like any other struct a vector is freed when it goes out of scope.
    {
        let _v = vec![1, 2, 3, 4];
    } // v goes out of scope and is freed here
}

fn strings() {
    // In Rust strings are a collection of bytes, with methods to provide additional functionalities.
    // Rust has two types of strings: `String` and `&str`.
    // The string slice `str`, `&str` in borrowed form, is a UTF-8 encoded string data stored in the binary.
    // The `String` type is a growable, mutable, owned, UTF-8 encoded string data type stored in the heap.
    // The `String` type is a wrapper over a `Vec<u8>`, so it implements all the methods of a vector.
    // I.e. the new method that creates an empty string in which data can be appended.
    let mut _s = String::new();
    // The function `to_string` is used to convert a string literal to a `String` when there is initial data.
    // Every type that implements the `Display` trait can be converted to a string.
    let data: &str = "Initial string";
    let _s: String = data.to_string();
    let _s: String = "Initial string".to_string();
    // This is equivalent to using the `String::from` function.
    let _s = String::from("Initial string");
    // A string can be updated and concatenated in many ways, such as the push_str method, the `+` operator, or the `format!` macro.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}, s1 is {s1}");
    // The `push_str` does not take ownership of the parameter.
    // Another way to concatenate string is using the `+` operator.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is no longer valid after this line
    println!("s2: {s2}, s3: {s3}");
    // The `+` operator uses the `fn add(self, s: &str) -> String` method, which takes ownership of the first string and a reference to the second string.
    // This method requires a reference of a string `&str` to add to a String, it is not possible to add two `String` together.
    // The compiler, though, can coerce the &String into a &str, then it turns &s2 into &s2[..] to get a slice of the whole string.
    // When concatenating multiple strings the `+` operator gets unwiedly
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
    // To make it more readable the `format!` macro can be used, it works as the println! macro.
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    // For indexing and slicing strings, the `[]` operator is not allowed because strings are a collection of bytes, not characters.
    // A String is a wrapper of Vec<u8> and not all the UTF-8 characters are one byte, such as the cyrillic characters (2 bytes each).
    // Indexing a string, then, would return a byte corresponding to the character, not the character.
    // In order to avoid confusion, Rust doesn't compile preventing misundertandings.
    let _hello = "Здравствуйте";
    // let first = &_hello[0]; // This doesn't work
    // Another reason Rust doesn't allow indexing is that it would take more than O(1) because Rust would need towalk through the String and determine the valid characters.
    // Indexing into a string, so, doesn't have a clear return type, since it could be a byte, a character, a grapheme cluster, or a string slice.
    // Rust allows to create string slices using the `&` operator and a range.
    // The range is a reference to the starting byte and the ending byte, but the ending byte is not included.
    // In the case of cyrillic characters, each character requires two bytes, so, to get two letters, it's necessary to get the range from 0 to 4.
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Two letters
    println!("{s}");
    // let s = &hello[0..1]; // Error: this would get half a letter, which is invalid
    // The best way to operate on strings is to use the `chars` method, which returns Unicode characters.
    // it can be usefulk to iterate over the characters of a string.
    for c in s.chars() {
        println!("{c}");
    }
    // In this case the type is explicit, bytes can be also used to get the raw bytes of a string.
    for b in s.bytes() {
        println!("{b}");
    }
}

fn hash_maps() {
    // `HashMap<K, V>` is a collection of keys and values, where each key is unique.
    // The mapping of keys of type `K` to values of type `V` is done via an hashing function that determines how it places the keys and values in memory.
    // Hash maps are useful when data needs to be accessed using a key instead of an index.
    // To create an empty hash map, the HashMap must be brought into scope because it is not in the prelude.
    // Similarly to strings, the new method is used to create an empty hash map.
    // Then the elmements are inserted using the `insert` method.
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // To access a value, the `get` method is used, which returns an Option<&V>.
    // The value is then copied to an Option<V> using the `copied` method.
    // The `unwrap_or` method is used to return a default value if the key is not found.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score of the {team_name} team: {score}");
    // HashMaps allow iteration over the keys and values.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    // The ownership of the elements of an hash map depends on the type of the key and value.
    // If the types implement the copy trait, such as i32, the values are copied into the hash map.
    // Otherwise, the values are moved into the hash map, which becomes the owner of the values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{field_name}, {field_value}"); // Invalid, the values are moved into the hash map
    // Since the keys of a hash map are unique, when inserting a value to an existing key, the value is replaced by default.
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    // To check if a key has a value, the `entry` method is used, which returns an `Entry` enum, that represents a value that may or may not exist.
    // The `or_insert` method is used to insert a value if the key doesn't have a value, otherwise it keeps the existing value.
    scores.entry(String::from("Blue")).or_insert(50); // This doesn't update the value
    println!("{scores:?}");
    // A value can be updated based on the existing value, because the `or_insert` method returns a mutable reference to the value.
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // The `split_whitespace` method returns an iterator over subslices, separated by spaces, of the value.
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count must be dereferenced to update the value.
        *count += 1;
    }
    println!("{map:?}");
}

fn excercises() {
    use std::collections::HashMap;
    {
        // Given a list of integers use a vector to return the median and the mode.
        fn median_mode(list: &mut Vec<i32>) {
            list.sort();
            let len = list.len();
            println!("median: {}", list[len / 2]); // it works even if len is odd.
            let mut map = HashMap::new();
            for i in list {
                let cnt = map.entry(i).or_insert(0);
                *cnt += 1;
            }
            println!(
                "Mode: {}",
                map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0
            )
        }

        let mut list = vec![1, 6, 1, 2, 4, 3, 8, 5, 9, 7, 1, 3, 2];

        median_mode(&mut list);
    }
    {
        // Convert a string to pig latin, so the first consonant of each word is moved to the end of the word and "ay" is added.
        fn pig_latin(s: &mut String) -> String {
            let vowels = ['a', 'e', 'i', 'o', 'u'];
            let mut res = s.clone();
            let first = s.chars().next().unwrap();
            if !vowels.contains(&first.to_ascii_lowercase()) {
                res.remove(0);
                res.push_str(&format!("-{first}ay")[..]);
            } else {
                res.push_str("-hay");
            }

            res
        }

        let mut s = String::from("Hello world");
        // let mut s = String::from("Apple");
        let res = pig_latin(&mut s);
        println!("String: {s}, Pig Latin: {res}");
    }
    {
        // Create a text interface to allow a user to add employee names to a department in a company, and list all people in a department or all people in the company.
        // Examples:
        // Add Sally to Engineering
        // List Engineering
        // List
        use std::io;

        fn add_employee(
            company: &mut HashMap<String, Vec<String>>,
            department: &str,
            employee: &str,
        ) {
            company
                .entry(department.to_string())
                .or_insert(vec![])
                .push(employee.to_string());
        }

        fn list_employees(company: &HashMap<String, Vec<String>>, department: &str) {
            match company.get(department) {
                Some(employees) => {
                    println!("Employees of the {department} department:");
                    for e in employees {
                        println!("{e}")
                    }
                }
                None => println!("Department {department} not found!"),
            }
        }

        fn list_all_employees(company: &HashMap<String, Vec<String>>) {
            for (dep, empl) in company {
                println!("Employees of the department {dep}:");
                for e in empl {
                    println!("{e}")
                }
                println!()
            }
        }

        let mut company = HashMap::new();

        loop {
            println!("Please enter a command:");
            let mut cmd = String::new();
            io::stdin()
                .read_line(&mut cmd)
                .expect("Failed to read line");

            let cmd: Vec<&str> = cmd.trim().split_whitespace().collect();

            match cmd[0] {
                "Add" => {
                    let employee = cmd[1];
                    let department = cmd[3];
                    add_employee(&mut company, department, employee);
                }
                "List" => {
                    if cmd.len() == 2 {
                        let department = cmd[1];
                        list_employees(&company, department);
                    } else {
                        list_all_employees(&company);
                    }
                }
                "Exit" => return,
                _ => {
                    println!("Invalid command");
                    continue;
                }
            }
        }
    }
}
