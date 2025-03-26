//! Functions in Rust.

pub(crate) fn main() {
    another_function(2);
    print_measure(5, 'm');
    statement();
    expression();
    println!(
        "The value of five is: {}, plus one: {}",
        five(),
        plus_one(five())
    );
}

fn another_function(x: i32) {
    println!("Value of x: {x}.");
}

fn print_measure(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}.");
}

fn statement() {
    let _y = 6;
    // let x = (let y = 6);
}

fn expression() {
    let y = {
        let x = 3;
        x + 1 // No semicolon here, meaning it's an expression and returns a value
    };

    println!("The value of y is: {y}");
}

fn five() -> i8 {
    5 // 5; would have been a statement
}

fn plus_one(x: i8) -> i8 {
    x + 1
}
