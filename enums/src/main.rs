/// Enums allow to define a type by enumerating its possible variants.
/// Enums are useful when we have a fixed set of values that we know at compile time.

fn main() {
    ip_addr_kind();
    message();
    option();
    match_control();
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

fn match_control() {}
