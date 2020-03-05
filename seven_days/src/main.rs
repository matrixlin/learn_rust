fn main() {
    println!("Hello, world!");
    let mut sum = 0;
    for x in 1..101 {
        sum = sum + x;
    }
    println!("The value of sum from 1 to 100 is {}", sum);
    calculate();
    add(1,100);
    add1();
    calculate2();
    let default_float = 1.0; //f64
    let default_integer = 12; //i32
    let a_integer = number1(default_integer);
    let a_float = number2(default_float);
    let tuple = tuple1(a_integer, a_float);
    println!("a_integer: {:?}, a_float: {:?}, tuple: {:?}", a_integer, a_float, tuple);
}
fn calculate() {
    let mut sum = 0;
    for x in 1..101 {
        sum = sum + x;
    }
    println!("The value of sum from 1 to 100 is {}", sum);
}
fn add(x: i32, y: i32) {
    let mut sum = 0;
    for x in x..=y {
        sum = sum + x;
    }
    println!("The value of sum from 1 to 100 is {}", sum);
}
fn add1() {
    let mut number =0;
    let mut sum = 0;
    while number != 100 {
        number += 1;
        sum = sum + number;
    }
    println!("the value of sum from 1 to 100 is {}", sum);
}
fn calculate2() {
    let mut counter = 0;
    let mut sum = 0;
    let result = loop {
        counter += 1;
        sum = sum + counter;
        if counter == 100 {
            break sum;
        }
    };
    println!("result: {}", result);
}
fn number1(x: i32) -> i32 {
    x
}
fn number2(x: f64) -> f64 {
    x
}
fn tuple1(x: i32, y: f64) -> (i32, f64) {
    (x, y)
}