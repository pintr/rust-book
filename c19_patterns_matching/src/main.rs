//! Patterns are a special syntax in Rust for matching against structure of types
//! Using patterns with `match` espression and other constructs gives more control on the program's control flow.
//! A pattern consist of combinations of: Literals, Destructured arrays, enums, structs, or tuples, Variables, Wildcards, and Placeholders
//! For example some patterns are `x`, `(a, 3)`, `Some(Color::Red)`
//! In contexts in which patterns are valid, these components describe the shape of data
//! A program matches the values against the patterns to determine whether it has the correct shape to run a piece of code.
//! To use a pattern, it is compared to some value and, if it matches its shape, that value can be used, for example using the `match` expression.

fn main() {
    patter_places();
    refutability();
    pattern_syntax();
}

fn patter_places() {
    // Patterns can be used in many places in a Rust program, here are all of them
    {
        // Arms of `match` expressions
        // `match` expressions are defined as the keyword `match`, a value to match on, and teh arms consisting of a pattern and an expression:
        // match VALUE {
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        //     PATTERN => EXPRESSION,
        // }
        // For example:
        let x = Some(1);
        match x {
            None => None,
            Some(i) => Some(i + 1),
        };
        // The patterns are `None` and `Some(i)` to the left
        // One requirement for the `match` expressions is that they need to be exhaustive, so all possibilities must be accounted for.
        // One possibility is using a catchall pattern for the last arm, e.g. a variable name matching any value that covers all the remaining cases.
        // The pattern `_` matches anything, but it doesn't bind to a variable, it can be useful to ignore any value not spacified. Mostly used as the last arm.
    }
    {
        // Conditional `if let` expressions
        // The `if let` expression is a shorter way to write a `match` that only matches one case.
        // The `if let` can be combined to other `if let`, `else if`, and `else if let`, giving more flexibility than `match` that only compares one value.
        // For example the choice of a bakground based ona series of checks:
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            // If there is a favourite colour, that one is used
            println!("Using your favorite color, {color}, as the background");
        } else if is_tuesday {
            // If no favourite color but it's tuesday, then green
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            // If no favourite colour and no tuesday consider the age
            if age > 30 {
                // If older then 30, purple is selected
                println!("Using purple as the background color");
            } else {
                // If younger then 30, orange is selected
                println!("Using orange as the background color");
            }
        } else {
            // If nothing specified, blue is selected
            println!("Using blue as the background color");
        }
        // `if let` expressions, additionally, introduce new variables that shadow existing variables
        // For example in `if let Ok(age) = age` introduces a new variable `age` with the value inside of the `Ok` variant, shadowing the existing `age` variable.
        // The inside is couldn't be added to the `if let` expressions because they compare two different values: a `Result` in the outside, a `u8` in the inside.
        // // The downside of `if let` is that the compiler doesn't check for exhaustiveness, if some cases miss the compiler would not alter to the possible bug.
    }
    {
        // Conditional loop `while let`
        // Similarly to `if let` the `while let` conditional loop allows a `while` loop to run for as long as a pattern matches
        // For example waiting a message sent between threads and checking a `Result` instead of an `Option`:
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(value) = rx.recv() {
            println!("{value}");
        }
        // The example prints `1`, `2`, and `3`, the `recv` takes the messages and returns a `Ok(value)`.
        // When the sender disconnects, it produces an `Err` ending the loop.
    }
    {
        // Loop `for`
        // Ina `for` loop, the value following `for` is a pattern:
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
        // The `enumerate` method on the iterator produces a value and its index in a tuple. First the index, then the value
    }
    {
        // The `let` statement
        // Even a basic `let` statement have a pattern: the variable name, and the expression: the value
        // let PATTERN = EXPRESSION
        let _x = 5;
        // In this example `x` is a pattern tha means "bind what matches here to the variable `x`"
        // With tuples is even clearer:
        let (_x, _y, _z) = (1, 2, 3);
        // In this case RUst compares the value `(1, 2, 3)` to the pattern `(_x, _y, _z)`
        // Since the value matches the pattern, the values are bound `1` to `_x`, `2` to `_y`, and `3` to `_z`
        // if the numebr of elements in the pattern doesn't match the number of elements in the tuple it gets a compiler error:
        // let (_x, _y) = (1, 2, 3);
        // To fix this error one or more values could be ignored using `_` or  `..`.
        let (_x, _y, _) = (1, 2, 3);
        let (_x, _y, ..) = (1, 2, 3);
        // In case of too many variables in the pattern those in excess need to be removed so they equals the number of elements.
    }
    {
        // Function parameters
        // The parameters of function can also be patterns:
        fn _foo(_x: i32) {
            // code goes here
        }
        // In this case the `_x` part is a pattern, similarly to `let`, here is an example with tuples:
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({x}, {y})");
        }

        let point = (3, 5);
        print_coordinates(&point);
        // `&point` matches the pattern `&(x, y): &(i32, i32)` so `x` is `3` and `y` is `5`.
    }
}

fn refutability() {
    // Patterns come in two forms:
    // - Refutable: patterns that can fail to match for some possible values, i.e. `Some(x)` in the `if let Some(x) = a_value` because if `a_value` is `None` it doesn't match.
    // - Irrefutable: patterns that match for any possible values, i.e. `x` in `let x = 5` because `x` matches anything
    // Function paramters, `let` statements, and `for` loops can only accept irrefutable patterns, otherwise the program can't do anything meaningful if they don't match.
    // The `if let`, `while let` expressionas, and `let...else` statement accept both patterns
    // But, in this, case, the compiler warns against irrefutabel patterns because they are intended to handle possible failure and a conditional has the ability to perform differently based on success or failure.
    // Most of the time it's not important the distinciton between the two
    // However it is useful to know the concept of refutability because it needs to change the used pattern or the construct.
    // Here is an example of using a refutable pattern `Some(x)` with an irrefutable one `let`:
    let some_option_value = Some(5);
    // let Some(x) = some_option_value;
    // If `some_option_value` were a None value it would fail to match `Some(x)`, but, since `let` only accepts irrefutable patterns, Rust gives error at compile time because not every valid value is covered.
    // With a refutable pattern when an irrefutable one is needed, it can be fixed by changing the pattern, i.e. instead of `let` use `if let`
    // If the pattern doesn't match, the code will skip the code in the curly brackets, granting to continue validly:
    let Some(_x) = some_option_value else {
        return;
    };
    // Now the code can continue but, if a irrefutable pattern is given to `if let`, such as `x` the compiler will give a warning:
    // let x = 5 else {
    //     return;
    // };
    // Rust complains because it doesn't make sense to use `if let` with  an irrefutable pattern.
    // FOr this reason `match` arms use refutable patterns, except for the last one, which is irrefutable.
    // Rust allows to use an irrefutable pattern with `match` but it's not very usefult because it could be substituted with a simpler `let` statement.
}

fn pattern_syntax() {}
