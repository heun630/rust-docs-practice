use std::io;

fn main() {
    float();
    numerical_computation();
    boolean();
    tup();
    array();
}

fn float() {
    let x = 2.0;
    let y: f32 = 3.0;

    println!("Value x is {x}");
    println!("Value y is {y}");
}

fn numerical_computation() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum: {sum}");
    println!("difference: {difference}");
    println!("product: {product}");
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");
    println!("remainder: {remainder}");
}

fn boolean() {
    let t = true;
    let f: bool = false;

    println!("t: {t}");
    println!("f: {f}");
}

fn tup() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
}

fn array() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}