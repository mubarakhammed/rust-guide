use std::io;
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "six";
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The value of SUBSCRIBER_COUNT is: {}", SUBSCRIBER_COUNT);

    // data type (coumpound type)

    let tup: (i32, f64, u8, &str) = (500, 6.4, 1, "Hello");
    let (x, y, z, a) = tup;
    println!("The value of y is: {}", y);
    println!("The value of a is: {}", a);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!("The value of six_point_four is: {}", six_point_four);

    my_function();
    another_function(5, 6);
    // control_flow();
    counter();
    whileLoop();
    forLoop();
}

fn my_function() {
    println!("Hello from my_function");
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = x + y;

    return sum;
}

fn control_flow() {
    println!("Please input a number.");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    if user_input == "5" {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn loop_example() {
    loop {
        println!("Just looping");
    }
}
fn counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 25 {
            break counter;
        }
    };
    println!("The result is: {}", result);
}

fn whileLoop() {
    let mut number = 3;

    while number < 20 {
        println!("{}!", number);
        number += 1;
    }

    println!("LIFTOFF!!!");
}

fn forLoop() {
    let a = [10, 20, 30, 40, 50];

    for items in a.iter() {
        println!("The value is: {}", items);
    }
}
