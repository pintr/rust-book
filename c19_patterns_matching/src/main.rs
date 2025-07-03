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

fn pattern_syntax() {
    // In this section there are all the valid syntax in patterns
    {
        // Matching literals:
        let x = 1;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
        // This code prints "one" because `x` equals 1.
        // This syntax is usefulwhen the code must take an action based on a particular concrete value.
    }
    {
        // Matching named variables
        // Named variables are irrefutable patterns that match any value.
        // There is a complication when used with `match`, `if let`, and `while let` becasue each start a new scope
        // In the new scope variables declared as part of a pattern inside the expression shadows those with the same name outside
        // Here is an example of shadowing using `x = Some(5)`, and `y = 10` then matched.
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),

            Some(y) => println!("Matched, y = {y}"),

            _ => println!("Default case, x = {x:?}"),
        }
        // The first arm doesn't match because the value inside `Some` is different
        // The second arm matches because the variable named `y` will match any value inside of `Some`
        // Since this is a new scope `y` is a new variable, different from the already defined `y = 10`
        // This means `y` binds to the inner value of the `Some` in `x`, in this case `5`
        // The only way to reach the last arm is by assigning `x` to `None`
        // Since `x` is not used in the pattern of the underscore arm, it is still the outer `x`
        println!("at the end: x = {x:?}, y = {y}");
        // Once the scope ends, and so does the scope of the inner `y`, the last `println!` produces the outer values.
        // To create a `match` that compares the values of the outer `x` and `y` it's needed a match guard conditional
    }
    {
        // Multiple patterns
        // Using the `|` syntax, which is the or operator, it is possible to match multiple patterns:
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("Anything else"),
        }
        // The code prints `one or two` with `x = 1` or `x = 2`
    }
    {
        // Matching ranges of values
        // The `..=` syntax is used to match an inclusive range of values:
        let x = 4;

        match x {
            1..=5 => println!("one through five"),
            _ => println!("anything else"),
        }
        // The first arm matches any number from 1 to 5, without `=` it would be from 1 to 4
        // The range match only works with numbers and chars, here is an example using chars:
        let x = 'c';

        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
        // In this case, with `x = 'c'`, the first arm matches.
    }
    {
        // Destructuring to break apart values
        // Patterns can also be used to destructure structs, enums, and tuples to use different parts of these values
        {
            // Destructuring Structs
            // The following struct `Point` has two fields: `x`, and `y`
            struct Point {
                x: i32,
                y: i32,
            }

            // Create a point with `x = 0` and `y = 7`
            let p = Point { x: 0, y: 7 };

            // The following code creates two variables `a` and `b` that match the calues of fields `x` and `y` of `p`
            let Point { x: a, y: b } = p;

            println!("Value of a: {a}");
            println!("Value of b: {b}");
            // This example shows that names of the variables don't have to match the field names, however it's common to match the fields' names
            // Because writing `let Point { x: x, y: y } = p;` contains duplication, Rust has a shorthand for patterns taht only list the fields:
            let Point { x, y } = p;
            println!("Value of x: {x}");
            println!("Value of y: {y}");
            // Now the variables `x` and `y` have been created and they match the fields
            // It is also possible to destructure with literal values as part of the struct pattern, this allows to test the fields for particular values:
            match p {
                Point { x, y: 0 } => println!("On the x axis at {x}"),
                Point { x: 0, y } => println!("On the y axis at {y}"),
                Point { x, y } => {
                    println!("On neither axis: ({x}, {y})");
                }
            }
            // Here is a `match` expression that separates `Point` values into three cases: point on `x`, point on `y`, or neither
            // The values `x = 0` and `y = 7` match the second arm
        }
        {
            // Destructuring Enums
            // Using the already seen `Message` enum match with patterns that will destructure each inner value:
            #[allow(dead_code)]
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(i32, i32, i32),
            }

            let msg = Message::ChangeColor(0, 160, 255);

            match msg {
                Message::Quit => {
                    println!("The Quit variant has no data to destructure.");
                }
                Message::Move { x, y } => {
                    println!("Move in the x direction {x} and in the y direction {y}");
                }
                Message::Write(text) => {
                    println!("Text message: {text}");
                }
                Message::ChangeColor(r, g, b) => {
                    println!("Change color to red {r}, green {g}, and blue {b}");
                }
            }
            // In this case the last arm is selected and the colour parameters are extracted
            // For the enum variant without data, such as `Message::Quit` the value can't be destructured any further
            // For struct-like enum variants, such as `Message::Move` the pattern is similar to matching structs by listing in curly brackets the fields with variables to break apart the pieces.
            // For tuple-like enum variants, such as `Message::Write` and `Message::ChangeColor`, the pattern is similar to matching tuples and the number of parameters match teh elements.
        }
        {
            // Destructuring Nested Structs and Enums
            // Since now the structs and enums were only one level deep, but matching works on nested items too:
            #[allow(dead_code)]
            enum Color {
                Rgb(i32, i32, i32),
                Hsv(i32, i32, i32),
            }

            #[allow(dead_code)]
            enum Message {
                Quit,
                Move { x: i32, y: i32 },
                Write(String),
                ChangeColor(Color),
            }

            let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

            match msg {
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    println!("Change color to red {r}, green {g}, and blue {b}");
                }
                Message::ChangeColor(Color::Hsv(h, s, v)) => {
                    println!("Change color to hue {h}, saturation {s}, value {v}");
                }
                _ => (),
            }
            // In this case the first arm of `match` matches the `Message::ChangeColor` variant that contains the `Color::Rgb` variant
            // The pattern then binds to the three inner `i32` values
            // The second arm, used in this example, instead matches the `Color::Hsv` instead in the same way.
        }
    }
    {
        // Ignoring values in a pattern
        // Sometimes is useful ignoring values in a apttern, such as the last arm of `match`, to catch all the possibilities that do nothing
        // There are different ways to ignore entire values of parts of values in a pattern using `_`
        {
            // Ignore an entire value with `_`
            // The underscore is a wildcard pattern that matches any value but it doesn't bind to the value, as in the last arm of a `match` expression
            // It can be used in many cases, such as funciton parameters:
            fn foo(_: i32, y: i32) {
                println!("This code only uses the y parameter: {y}");
            }

            foo(3, 4);
            // In this casethe function `foo` will ignore the first parameter, even if, most of the time, the signature of the function gets changed
            // it can be useful for example when implementing a trait with a certain signature but the funciton body doesn't need one of the parameters, this avoids compiler warnings.
        }
        {
            // Ignore parts of a value with nested `_`
            // The `_` wildcard can be used inside of another pattern to ignore part of a value, for example when there is the need to only test part of a value.
            // For example when a setting's value can be unset and resetted, but cannot be overwritten:
            let mut setting_value = Some(5);
            let new_setting_value = Some(10);

            match (setting_value, new_setting_value) {
                (Some(_), Some(_)) => {
                    println!("Can't overwrite an existing customized value");
                }
                _ => {
                    setting_value = new_setting_value;
                }
            }

            println!("setting is {setting_value:?}");
            // In this case, if the `setting_value` isn't `None` it won't be overwritten, otherwise is set as `new_setting_value`
            // Underscores can alse be used in multiple places to ignore particular values:
            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, _, third, _, fifth) => {
                    println!("Some numbers: {first}, {third}, {fifth}");
                }
            }
            // This code will ignore the values `4` and `16`
        }
        {
            // Unused variable by starting with `_`
            // If a variable is created but not used anywhere, Rust will issue a warning. It can be useful for prototyping, though
            // Rust can be told not to warn about it by starting the name with `_`:
            let _x = 5;
            let _y = 10;
            // If the two variables didn't start with `_` they would give a warning
            // Anyway, differently from `_`, this notation binds the value to a variable:
            let s = Some(String::from("Hello!"));

            if let Some(_s) = s {
                println!("found a string");
            }

            // Trying to print `s` would result in a compiler error because `_s` binds the value of `s`, changing `_s` with `_` works:
            let s = Some(String::from("Hello!"));

            if let Some(_) = s {
                println!("found a string");
            }

            println!("{s:?}");
            // `s` was never moved, so it's still available
        }
        {
            // Ignore the remaining parts of a value using `..`
            // With values composed by many parts it is possible to use the `..` syntax to use specific parts and ignore the rest, avoiding to use many `_`
            // The `..` pattern ignores any parts of a value that haven't explicitly matchedin the rest of the pattern:
            struct Point {
                _x: i32,
                y: i32,
                _z: i32,
            }

            let origin = Point { _x: 0, y: 0, _z: 0 };

            match origin {
                Point { y, .. } => println!("y is {y}"),
            }
            // In this case only `y` is considered, the rest is ignored.
            // This is particularly useful with structs with many fields, since it allows to select only the needed ones
            // This syntax can be used for tuples as well:
            let numbers = (2, 4, 8, 16, 32);

            match numbers {
                (first, .., last) => {
                    println!("Some numbers: {first}, {last}");
                }
            }
            // In this case the first and last values are matched, while all the others in the middle are ignored
            // Anyway, using `..` must be unambiguous and clear which values are intended for matching, and which should be ignored, otherwise Rust gives an error:
            // let numbers = (2, 4, 8, 16, 32);

            // match numbers {
            //     (.., second, ..) => {
            //         println!("Some numbers: {second}")
            //     }
            // }
            // In this case it's impossible for Rust to determine how many values in the tuple ignoring before matching `second`
            // The variable name doesn't mean anything meaningful to Rust
        }
    }
    {
        // Extra conditionals with Match Guards
        // A match guard is an additional `if` condition specified after the pattern in a `match` arm, that must also match for that arm to be chosen:
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
        // In this case the first arm is selected only if it exists and is even
        // If the value is odd, the second arm would had been chosen, otherwise the choice would be the last arm.
        // There is no way to express the even check condition within a pattern, so the match guards allows to express this logic.
        // The downside is that the compiler doesn't check for exhaustiveness when there are match guards
        // In the example of pattern-shadowing a match guard could solve the problem of testing on the outer varaible:
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
        // Now, with `x` different to `50` and `y` it entrers the dafault case because `y` isn't shadowed, and it can be used in the match guard.
        // Using `Some(y)` would have shadowed the outer `y`, but this time `n` doesn't shadow anything because there is no outer `n`
        // The match guard `if n == y` is not a pattern and doesn't introduce new variables: this `y` is the outer `y`
        // In amatch guard it is possible to use tho or operator `|` to specify multiple patterns, and the match guard will apply to all of the patterns:
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
        // In this case `if y` applies to `4`, `5`, and `6`, not just `6` so it's like: `(4 | 5 | 6) if y`
        // For this reason with `y = false` the match prints `no`, with `y = true` instead it would have printed `yes`
    }
    {
        // The at `@` bindings
        // The `@` operator lets creating a variable that holds a value at the same time it's tested for a pattern match
        // For example testing the `id` field of a variant of an enum `Message::Hello` by binding it to another variable `id_variable`
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {id_variable}"),
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {id}"),
        }
        // This example prints `Found an id in range: 5`
        // By specifying `id_variable @` before the range, tha value matching the range is captured while testing if it matches the range too.
        // In the second arm, instead, it doesn't have a variable that contains the actual value of the `id` field
        // The id could go from 10 to 12 but the code wouldn't know its real value of `id`
        // In a nutshell `@` allows to test a value and save it in a variable within one pattern.
    }
}
