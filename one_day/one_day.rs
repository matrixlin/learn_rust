fn main() {
   
    println!("hello world");


    let number = 8;
    println!("The value of number is {}", number);

    //number = 9;

    let mut num = 10;

    num = 11;

    println!("The value of num is {}", num);

    let a_a_1 = 1;

    let _a1 = 2;


    //shadowing
    let a = 12;
    let a = 15;
    println!("THe value of a is {}.",a);

    //constants

    const POINT: f64 = 0.0;
    println!("The value of POINT is {}.", POINT);
}
