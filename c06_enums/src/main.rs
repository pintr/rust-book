/// Enums allow to define a type by enumerating its possible variants.
/// Enums are useful when we have a fixed set of values that we know at compile time.

fn main() {
    ip_addr_kind();
    message();
    option();
    match_control();
    if_let();
}

#[allow(dead_code)]
fn ip_addr_kind() {
    {
        // An example of an enum is the IpAddrKind enum that defines the possible IP address versions: V4 and V6.
        // Enums in this case does not have any data associated with its variants, so we can tackle this using a struct.
        enum IpAddrKind {
            V4,
            V6,
        }

        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let _four = IpAddrKind::V4;
        let _six = IpAddrKind::V6;

        let _home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let _loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }
    {
        // We can also define the IpAddr enum to have data associated with its variants.
        // In this way the structu is avoided, and the solution is more coincise.
        // We can put data directly in the enum variants.
        enum IpAddr {
            V4(String),
            V6(String),
        }

        let _home = IpAddr::V4(String::from("127.0.0.1"));
        let _loopback = IpAddr::V6(String::from("::1"));
    }
    {
        // Another advantage is that we can have different types and amounts of associated data
        // Any kind of data can be associated with an enum variant: strings, numeric types, structs, or even other enums.
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let _home = IpAddr::V4(127, 0, 0, 1);
        let _loopback = IpAddr::V6(String::from("::1"));
    }
}

fn message() {
    //! Message is an enum with four variants: Quit, Move, Write, and ChangeColor.
    // Quit has no data associated with it at all.
    // Move includes an anonymous struct inside it.
    // Write includes a single String.
    // ChangeColor includes a tuple composed by three i32 values.
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        _Quit,
        _Move { x: i32, y: i32 },
        Write(String),
        _ChangeColor(i32, i32, i32),
    }

    // The Message enum holds the same data as the following structs, but all the variants are grouped togheter.
    struct _QuitMessage;
    struct _MoveMessage {
        x: i32,
        y: i32,
    }
    struct _WriteMessage(String);
    struct _ChangeColorMessage(i32, i32, i32);

    impl Message {
        fn call(&self) {
            println!("{:?}", self);
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn option() {
    //! The Option enum is defined by the standard library and is used when absence is a possibility.
    // A value of Option<T> can either be Some(T) or None.
    // For example  requesting the first item of a list depends if it's empy or not
    // Rust doesn't have nulls, because is a value that means absence of a value, so a variable can be null or not null.
    // If there is use of a null value as a non-null value, the program will crash.

    // The Option<T> is defined as:
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }
    // Option is included in the prelude, so it doesn't require to be included in the code.
    let _number = Some(5);
    let _char = Some('a');
    let _absent_number: Option<i32> = None;
    // Rust can infer the types of the Some variants, but not for the None variant, so we need to explicitly define the type.
    // The advantage of using Option<T> where T can be any type, is that T and Option<T> are different types, and the compiler will check if the types are used correctly.
    // If we try to use an Option<T> value as a T value, the compiler will throw an error.
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);

    // let num = _x + _y; // This will throw an error because we are trying to add an i8 and an Option<i8>
    // Option<T> needs to be converted to T before using it.
    // This catches the most common problem of null: assuming that a value isn't null when it actually is.
    // In order to use Option<T> the code needs to handle each variant, so it will run only when there is Some(T), or when there is a None.
}

#[allow(dead_code)]
fn match_control() {
    //! The match control flow operator is used to compare a value against a series of patterns and then execute code based on which pattern matches.
    // The match operator is similar to a switch statement in other languages.
    // The match expression acts like a coin sorting machine where the coin falls in the first matching slot.
    // Match is composed by arms separated by a comma, each arm has a pattern and the code to run if the value matches the pattern, separated by =>.
    // The match arms are evaluated from top to bottom, and the first match is executed.
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => {
                    // Code to run if the value matches the pattern
                    println!("Lucky penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        let penny = Coin::Penny;
        println!("Value in cents of a penny: {}", value_in_cents(penny))
    }
    {
        // An enum can also have data associated with its variants.
        // In this case, the match operator can destructure the data and use it in the code.
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }

        let quarter = Coin::Quarter(UsState::Alaska);
        println!("Value in cents of a quarter: {}", value_in_cents(quarter))
    }
    {
        // The match expression allows to compare the variants of Option<T too, for example in the case of Option<i32>.
        // The match operator can be used to handle the Some(T) and None variants, for example to add one to the value of an Option<i32>.

        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                Some(i) => Some(i + 1),
                None => None,
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("Six: {:?}, None: {:?}", six, none);
    }
    {
        // The arms of a match expression must handle all the possible values of the value being matched.
        // So matches in Rust are exhaustive, and the compiler will check if all the possible cases are handled.
        // If the compiler can't be sure that all possible cases are handled, it will throw an error.
        // fn plus_one(x: Option<i32>) -> Option<i32> {
        //     match x {
        //         // None is not covered, so the compiler gives an error
        //         Some(i) => Some(i + 1),
        //     }
        // }
        // Rust provides a catch-all patterns for the cases that are not explicitly handled
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        fn add_fancy_hat() {
            println!("Adding fancy hat");
        }
        fn remove_fancy_hat() {
            println!("Removing fancy hat");
        }
        fn move_player(num_spaces: u8) {
            println!("Moving player {} spaces", num_spaces);
        }
        // In this case all the values that are not 3 or 7 fall in the `other` arm.
        // There is a value for the default case without the need to use the value: _.
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
        fn reroll() {
            println!("Rerolling the dice");
        }
        // In this case the exhaustive check is satisfied by the _.
        // If we want the default value to do nothing, we can use the empty tuple ().
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }
    }
}

fn if_let() {
    //! if let is a construct that allows to match a single pattern, and ignore the rest.
    // It is useful when we are interested in only one pattern, and we don't want to list all the possible patterns.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("Max: {}", max),
        _ => (),
    }
    // The same can be achieved with if let
    // The if let construct takes a pattern and an expression separated by an equal sign.
    if let Some(max) = config_max {
        println!("Max: {}", max);
    }
    // In this case the pattern is Some(max), and max binds to the value of Some.
    // max can be used in the body of the if let block.
}
