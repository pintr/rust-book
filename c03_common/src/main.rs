//! Commmon programming concepts in Rust.

mod control;
mod functions;

fn main() {
    mutability();
    shadow();
    operations();
    chars();
    tuples();
    arrays();

    functions::main();
    control::main();
}

fn mutability() {
    // Demonstrates variable mutability in Rust.
    // This function creates a mutable variable `x`, prints its initial value,
    // modifies the value of `x`, and then prints the updated value.

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}

fn shadow() {
    // Demonstrates variable shadowing in Rust.
    // This function creates a variable `x`, shadows it with a new value,
    // and then shadows it again within an inner scope. It prints the value
    // of `x` at different points to show how shadowing works. Additionally,
    // it demonstrates shadowing with a variable of a different type.

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();

    println!("The value of spaces is: {spaces}");
}

fn operations() {
    // Demonstrates basic operations in Rust.

    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient: f32 = 56.7 / 32.2;
    let _remainder = 43 % 5;
    let _t = true;
    let _f: bool = false;
}

fn chars() {
    // Demonstrates character types in Rust.

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
}

fn tuples() {
    // Demonstrates tuples in Rust.

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = _tup;
    let _five_hundred = _tup.0;
    let _six_point_four = _tup.1;
    let _one = _tup.2;

    println!("First element: {_x} = {_five_hundred}");
    println!("Second element: {_y} = {_six_point_four}");
    println!("Third element: {_z} = {_one}");
}

fn arrays() {
    // Demonstrates arrays in Rust.

    let _a = [1, 2, 3, 4, 5];
    let _b = [2; 5];
    for e in _b {
        println!("{e}");
    }
    println!("Second element: {}", _a[1]);

    println!("Please enter an array index.");
    let mut i = String::new();
    std::io::stdin()
        .read_line(&mut i)
        .expect("Failed to read line");

    let i: usize = match i.trim().parse() {
        Ok(num) if (0.._a.len()).contains(&num) => num,
        Ok(_) => {
            println!("Index out of bounds.");
            return;
        }
        Err(_) => {
            println!("Invalid index.");
            return;
        }
    };

    println!("The value of the element at index {i} is: {}", _a[i]);
}
