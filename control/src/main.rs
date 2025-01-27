fn main() {
    if_else(2);
    multiple_conditions(6);
    if_let(true);
    // infinite_loop();
    loop_value();
    loop_label();
    loop_conditional();
    loop_collection();
    println!("Converted: {}", convert_temperature(0.0, false));
    let n_fib = 5;
    println!("Fibonacci of {n_fib}: {}", fibonacci(n_fib));
}

fn if_else(num: i32) {
    // the condition must be boolean
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn multiple_conditions(num: i32) {
    // This can become hard to read, better to use match
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }
}

fn if_let(cond: bool) {
    let n = if cond { 5 } else { 6 }; // Must be always the same type (e.g. i32)

    println!("The value of number is: {n}");
}

#[allow(dead_code)]
fn infinite_loop() {
    loop {
        println!("again!");
    }
}

fn loop_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");
}

fn loop_label() {
    let mut count = 0;
    'outer: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 5 {
                break;
            }

            if count == 2 {
                break 'outer;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Exited the outer loop");
}

fn loop_conditional() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_collection() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for n in (1..4).rev() {
        println!("{n}");
    }
    println!("LIFTOFF!!!");
}

fn convert_temperature(temp: f64, f_to_c: bool) -> f64 {
    if f_to_c {
        (temp - 32.0) * 5.0 / 9.0
    } else {
        (temp * 9.0 / 5.0) + 32.0
    }
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let mut f = 0;
    let mut s = 1;
    let mut res = 0;

    for _ in 2..n {
        res = f + s;
        f = s;
        s = res;
    }

    res
}
